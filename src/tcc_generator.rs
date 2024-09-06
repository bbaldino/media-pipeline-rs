use rtp_parse::rtp::tcc_header_extension::get_tcc_seq_num;

use crate::{
    packet_handler::{PacketObserver, SomePacketHandler},
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

impl PacketObserver for TccGenerator {
    fn observe(&mut self, packet_info: &PacketInfo) {
        let rtp_packet = match packet_info.packet {
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

impl From<TccGenerator> for SomePacketHandler {
    fn from(value: TccGenerator) -> Self {
        SomePacketHandler::PacketObserver(Box::new(value))
    }
}
