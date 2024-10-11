use data_pipeline::data_handler::{DataFilter, SomeDataHandler};

use crate::packet_info::PacketInfo;

#[derive(Default)]
pub struct DiscardableDiscarder;

impl DataFilter<PacketInfo> for DiscardableDiscarder {
    fn should_forward(&mut self, data: &PacketInfo) -> bool {
        !data.should_discard
    }
}

impl From<DiscardableDiscarder> for SomeDataHandler<PacketInfo> {
    fn from(value: DiscardableDiscarder) -> Self {
        SomeDataHandler::Filter(Box::new(value))
    }
}
