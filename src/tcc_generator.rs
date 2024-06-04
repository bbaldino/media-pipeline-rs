use rtp_rs::rtp::tcc_header_extension::get_tcc_seq_num;

use crate::{
    node::{NextNode, Node},
    packet_info::{PacketInfo, SomePacket},
};

#[derive(Default)]
pub struct TccGenerator {
    next: NextNode,
}

impl Node for TccGenerator {
    fn process_packet(&mut self, packet_info: PacketInfo) {
        match packet_info.packet {
            SomePacket::RtpPacket(ref rtp) => {
                if let Some(tcc) = rtp.header.extensions.iter().find(|e| e.has_id(5)) {
                    let seq_num = get_tcc_seq_num(tcc);
                    println!("got tcc seq num {seq_num}");
                    // TODO: rest of tcc feedback generation
                }
                self.next.process_packet(packet_info);
            }
            _ => panic!("TccGenerator shouldn't see non rtp packet"),
        }
    }

    fn attach(&mut self, next: Box<dyn Node>) {
        self.next.replace(next);
    }
}
