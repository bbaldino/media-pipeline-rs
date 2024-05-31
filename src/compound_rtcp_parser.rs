use bitcursor::bit_cursor::BitCursor;
use rtp_rs::rtcp::rtcp_packet::parse_rtcp_packet;

use crate::{
    node::{NextNode, Node},
    packet_info::{PacketInfo, SomePacket},
};

#[derive(Default)]
pub struct CompoundRtcpParser {
    next: NextNode,
}

impl Node for CompoundRtcpParser {
    fn process_packet(&mut self, mut packet_info: PacketInfo) {
        let packet_buf = match packet_info.packet {
            SomePacket::UnparsedPacket(buf) => buf,
            _ => {
                println!(
                    "rtcp parser got invalid packet type: {:?}",
                    packet_info.packet
                );
                return;
            }
        };
        let mut cursor = BitCursor::from_vec(packet_buf);

        if let Ok(compound_rtcp) = parse_rtcp_packet(&mut cursor)
            .inspect_err(|e| println!("Error parsing compound RTCP packet: {e}"))
        {
            packet_info.packet = SomePacket::RtcpPacket(compound_rtcp);
            self.next.process_packet(packet_info);
        }
    }

    fn attach(&mut self, next: Box<dyn Node>) {
        self.next.replace(next);
    }
}
