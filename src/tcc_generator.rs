use rtp_rs::rtp::tcc_header_extension::get_tcc_seq_num;

use crate::{
    node::{PacketObserver, SomePacketHandler},
    packet_info::{PacketInfo, SomePacket},
};

#[derive(Default)]
pub struct TccGenerator;

impl PacketObserver for TccGenerator {
    fn observe(&mut self, packet_info: &PacketInfo) {
        match packet_info.packet {
            SomePacket::RtpPacket(ref rtp) => {
                if let Some(tcc) = rtp.header.extensions.iter().find(|e| e.has_id(5)) {
                    let seq_num = get_tcc_seq_num(tcc);
                    println!("got tcc seq num {seq_num}");
                    // TODO: rest of tcc feedback generation
                }
            }
            _ => panic!("TccGenerator shouldn't see non rtp packet"),
        }
    }
}

impl From<TccGenerator> for SomePacketHandler {
    fn from(value: TccGenerator) -> Self {
        SomePacketHandler::PacketObserver(Box::new(value))
    }
}
