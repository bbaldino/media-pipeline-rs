use crate::{
    packet_handler::{PacketObserver, SomePacketHandler},
    packet_info::PacketInfo,
};

pub struct PacketLogger;

impl PacketObserver for PacketLogger {
    fn observe(&mut self, packet_info: &PacketInfo) {
        println!("packet: {}", packet_info.packet);
    }
}

impl From<PacketLogger> for SomePacketHandler {
    fn from(value: PacketLogger) -> Self {
        SomePacketHandler::PacketObserver(Box::new(value))
    }
}
