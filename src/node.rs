use anyhow::Result;
use serde_json::json;

use crate::packet_info::PacketInfo;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub trait PacketObserver {
    fn observe(&mut self, packet_info: &PacketInfo);
}

pub trait PacketTransformer {
    fn transform(&mut self, packet_info: PacketInfo) -> Result<PacketInfo>;
}

pub trait PacketFilter {
    fn should_forward(&mut self, packet_info: &PacketInfo) -> bool;
}

impl<F> PacketFilter for F
where
    F: FnMut(&PacketInfo) -> bool,
{
    fn should_forward(&mut self, packet_info: &PacketInfo) -> bool {
        (self)(packet_info)
    }
}

pub trait PacketConsumer {
    fn consume(&mut self, packet_info: PacketInfo);
}

pub trait PacketDemuxer {
    fn find_path(&mut self, packet_info: &PacketInfo) -> Option<&mut dyn Node>;
    // PacketDemuxer has to have its own visitor logic since it handles its own paths
    fn visit(&mut self, visitor: &mut dyn NodeVisitor);
}

pub enum SomePacketHandler {
    PacketObserver(Box<dyn PacketObserver>),
    PacketTransformer(Box<dyn PacketTransformer>),
    PacketFilter(Box<dyn PacketFilter>),
    PacketConsumer(Box<dyn PacketConsumer>),
    PacketDemuxer(Box<dyn PacketDemuxer>),
}

pub trait NodeVisitor {
    fn visit(&mut self, node: &mut dyn Node);
}

#[derive(Default, Debug)]
pub struct StatsNodeVisitor {
    all_stats: serde_json::Value,
}

impl NodeVisitor for StatsNodeVisitor {
    fn visit(&mut self, node: &mut dyn Node) {
        let stats = node.get_stats();
        self.all_stats[node.name()] = stats;
    }
}

pub trait Node {
    fn name(&self) -> String;
    fn process_packet(&mut self, packet_info: PacketInfo);
    fn attach(&mut self, next: Box<dyn Node>);
    fn get_stats(&self) -> serde_json::Value;
    fn visit(&mut self, visitor: &mut dyn NodeVisitor);
}

pub struct DefaultNode {
    name: String,
    packets_ingress: u32,
    packets_egress: u32,
    packets_discarded: u32,
    handler: SomePacketHandler,
    next: NextNode,
}

impl DefaultNode {
    pub fn from_handler<T: Into<String>, U: Into<SomePacketHandler>>(
        name: T,
        handler: U,
    ) -> Box<Self> {
        Box::new(Self {
            name: name.into(),
            handler: handler.into(),
            next: NextNode::default(),
            packets_ingress: 0,
            packets_egress: 0,
            packets_discarded: 0,
        })
    }
}

impl Node for DefaultNode {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn process_packet(&mut self, packet_info: PacketInfo) {
        self.packets_ingress += 1;
        match self.handler {
            SomePacketHandler::PacketObserver(ref mut o) => {
                o.observe(&packet_info);
                self.packets_egress += 1;
                self.next.process_packet(packet_info);
            }
            SomePacketHandler::PacketTransformer(ref mut t) => match t.transform(packet_info) {
                Ok(transformed) => {
                    self.packets_egress += 1;
                    self.next.process_packet(transformed);
                }
                Err(e) => {
                    self.packets_discarded += 1;
                    println!("Packet transformer failed: {e}");
                }
            },
            SomePacketHandler::PacketFilter(ref mut f) => {
                if f.should_forward(&packet_info) {
                    self.packets_egress += 1;
                    self.next.process_packet(packet_info);
                } else {
                    self.packets_discarded += 1;
                }
            }
            SomePacketHandler::PacketDemuxer(ref mut d) => {
                if let Some(path) = d.find_path(&packet_info) {
                    self.packets_egress += 1;
                    path.process_packet(packet_info);
                } else {
                    self.packets_discarded += 1;
                }
            }
            SomePacketHandler::PacketConsumer(ref mut c) => {
                c.consume(packet_info);
            }
        }
    }

    fn attach(&mut self, next: Box<dyn Node>) {
        match self.handler {
            SomePacketHandler::PacketDemuxer(_) => panic!("Can't attach to a Demuxer"),
            _ => self.next.replace(next),
        };
    }

    fn get_stats(&self) -> serde_json::Value {
        json!({
            "packets_ingress": self.packets_ingress,
            "packets_egress": self.packets_egress,
            "packets_discarded": self.packets_discarded
        })
    }

    fn visit(&mut self, visitor: &mut dyn NodeVisitor) {
        visitor.visit(&mut *self);
        match self.handler {
            SomePacketHandler::PacketDemuxer(ref mut d) => d.visit(visitor),
            SomePacketHandler::PacketConsumer(_) => {}
            _ => self.next.visit(visitor),
        };
    }
}

/// Helper type that can be used for shared data in nodes
pub struct SharedData<T>(Arc<RwLock<T>>);

// Deriving clone for SharedData doesn't seem to get picked up correctly when trying to derive
// clone for a struct which contains a SharedData, so manually implement it here.
impl<T> Clone for SharedData<T> {
    fn clone(&self) -> Self {
        SharedData(self.0.clone())
    }
}

impl<T> Default for SharedData<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(Arc::new(RwLock::new(T::default())))
    }
}

impl<T> SharedData<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }

    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        // An interesting note
        // [here](https://github.com/dtolnay/anyhow/issues/81#issuecomment-609171265) on why anyhow
        // doesn't work with this error and why posion errors with mutexes should always panic
        self.0.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().unwrap()
    }
}

/// Helper type to model an optional next node in the chain
#[derive(Default)]
pub struct NextNode(Option<Box<dyn Node>>);

impl NextNode {
    pub fn process_packet(&mut self, packet_info: PacketInfo) {
        if let Some(ref mut n) = self.0 {
            n.process_packet(packet_info);
        }
    }

    pub fn replace(&mut self, new: Box<dyn Node>) -> Option<Box<dyn Node>> {
        self.0.replace(new)
    }

    pub fn take(&mut self) -> Option<Box<dyn Node>> {
        self.0.take()
    }

    pub fn visit(&mut self, visitor: &mut dyn NodeVisitor) {
        if let Some(ref mut n) = self.0 {
            n.visit(visitor);
        }
    }
}
