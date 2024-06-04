use anyhow::Context;
use rtp_rs::rtp::rtp_packet::read_rtp_packet;

use crate::{
    node::{NextNode, Node},
    packet_info::{PacketInfo, SomePacket},
};

#[derive(Default)]
pub struct RtpParser {
    next: NextNode,
}

impl Node for RtpParser {
    fn process_packet(&mut self, mut packet_info: PacketInfo) {
        match packet_info.packet {
            SomePacket::UnparsedPacket(data) => {
                if let Ok(packet) = read_rtp_packet(data)
                    .context("rtp parse")
                    .inspect_err(|e| println!("Error parsing rtp: {e}"))
                {
                    println!("parsed rtp packet {packet:x?}");
                    packet_info.packet = SomePacket::RtpPacket(packet);
                    self.next.process_packet(packet_info);
                }
            }
            _ => println!(
                "Rtp parser got unexpected packet type {:?}",
                packet_info.packet
            ),
        }
    }

    fn attach(&mut self, next: Box<dyn Node>) {
        self.next.replace(next);
    }
}
