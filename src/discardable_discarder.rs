use crate::{
    packet_handler::{PacketFilter, SomePacketHandler},
    packet_info::PacketInfo,
};

#[derive(Default)]
pub struct DiscardableDiscarder;

impl PacketFilter for DiscardableDiscarder {
    fn should_forward(&mut self, packet_info: &PacketInfo) -> bool {
        !packet_info.should_discard
    }
}

impl From<DiscardableDiscarder> for SomePacketHandler {
    fn from(value: DiscardableDiscarder) -> Self {
        SomePacketHandler::PacketFilter(Box::new(value))
    }
}
