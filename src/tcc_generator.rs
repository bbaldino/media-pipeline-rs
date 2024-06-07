use rtp_rs::rtp::tcc_header_extension::get_tcc_seq_num;

use crate::{
    node::{PacketObserver, SomePacketHandler},
    packet_info::{PacketInfo, SomePacket},
};

#[derive(Default)]
pub struct TccGenerator;

impl PacketObserver for TccGenerator {
    fn observe(&mut self, packet_info: &PacketInfo) {
        let rtp_packet = match packet_info.packet {
            SomePacket::AudioRtpPacket(ref rtp) => rtp,
            SomePacket::VideoRtpPacket(ref rtp) => rtp,
            _ => panic!("TccGenerator shouldn't see non rtp packet"),
        };

        if let Some(tcc) = rtp_packet.get_extension_by_id(5) {
            let seq_num = get_tcc_seq_num(tcc);
            println!("got tcc seq num {seq_num}");
            // TODO: rest of tcc feedback generation
        }
    }
}

impl From<TccGenerator> for SomePacketHandler {
    fn from(value: TccGenerator) -> Self {
        SomePacketHandler::PacketObserver(Box::new(value))
    }
}
