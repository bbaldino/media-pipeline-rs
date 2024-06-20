use anyhow::Result;

use crate::{
    node::{Node, NodeVisitor},
    packet_info::PacketInfo,
};

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
