use crate::{
    node::{Node, NodeVisitor},
    packet_handler::{PacketDemuxer, SomePacketHandler},
    packet_info::PacketInfo,
};

pub type Predicate<T> = dyn Fn(&T) -> bool;

pub struct ConditionalPath<T, U> {
    pub predicate: Box<Predicate<T>>,
    pub next: U,
}

/// A Demuxer whose paths are fixed and known at creation time, so that they don't need to be
/// locked for each packet.
#[derive(Default)]
pub struct StaticDemuxer {
    packet_paths: Vec<ConditionalPath<PacketInfo, Box<dyn Node>>>,
}

impl StaticDemuxer {
    pub fn new(packet_paths: Vec<ConditionalPath<PacketInfo, Box<dyn Node>>>) -> Self {
        StaticDemuxer { packet_paths }
    }
}

impl PacketDemuxer for StaticDemuxer {
    fn find_path(&mut self, packet_info: &PacketInfo) -> Option<&mut dyn Node> {
        for path in &mut self.packet_paths {
            if (path.predicate)(packet_info) {
                return Some(&mut *path.next);
            }
        }
        None
    }

    fn visit(&mut self, visitor: &mut dyn NodeVisitor) {
        for path in &mut self.packet_paths {
            path.next.visit(visitor);
        }
    }
}

impl From<StaticDemuxer> for SomePacketHandler {
    fn from(value: StaticDemuxer) -> Self {
        SomePacketHandler::PacketDemuxer(Box::new(value))
    }
}
