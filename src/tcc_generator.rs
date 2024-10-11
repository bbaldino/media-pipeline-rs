use data_pipeline_rs::data_handler::{DataObserver, SomeDataHandler};
use rtp_parse::rtp::tcc_header_extension::get_tcc_seq_num;

use crate::{
    packet_info::{PacketInfo, SomePacket},
    util::LiveStateReader,
};

pub const TCC_URI: &str =
    "http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01";

pub struct TccGenerator {
    tcc_ext_id: LiveStateReader<Option<u8>>,
}

impl TccGenerator {
    pub fn new(tcc_ext_id: LiveStateReader<Option<u8>>) -> Self {
        TccGenerator { tcc_ext_id }
    }
}

impl DataObserver<PacketInfo> for TccGenerator {
    fn observe(&mut self, data: &PacketInfo) {
        let rtp_packet = match data.packet {
            SomePacket::AudioRtpPacket(ref rtp) => rtp,
            SomePacket::VideoRtpPacket(ref rtp) => rtp,
            _ => panic!("TccGenerator shouldn't see non rtp packet"),
        };

        if let Some(tcc_ext_id) = *self.tcc_ext_id.value() {
            if let Some(tcc) = rtp_packet.get_extension_by_id(tcc_ext_id) {
                let _seq_num = get_tcc_seq_num(tcc);
                // TODO: rest of tcc feedback generation
            }
        }
    }
}

impl From<TccGenerator> for SomeDataHandler<PacketInfo> {
    fn from(value: TccGenerator) -> Self {
        SomeDataHandler::Observer(Box::new(value))
    }
}
