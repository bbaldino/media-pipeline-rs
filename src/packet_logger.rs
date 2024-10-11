use data_pipeline::data_handler::{DataObserver, SomeDataHandler};

use crate::packet_info::PacketInfo;

pub struct PacketLogger;

impl DataObserver<PacketInfo> for PacketLogger {
    fn observe(&mut self, data: &PacketInfo) {
        println!("packet: {}", data.packet);
    }
}

impl From<PacketLogger> for SomeDataHandler<PacketInfo> {
    fn from(value: PacketLogger) -> Self {
        SomeDataHandler::Observer(Box::new(value))
    }
}
