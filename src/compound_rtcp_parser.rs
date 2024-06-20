use anyhow::{Context, Result};
use bitcursor::bit_cursor::BitCursor;
use rtp_rs::rtcp::rtcp_packet::parse_rtcp_packet;

use crate::{
    packet_handler::{PacketTransformer, SomePacketHandler},
    packet_info::{PacketInfo, SomePacket},
};

#[derive(Default)]
pub struct CompoundRtcpParser;

impl PacketTransformer for CompoundRtcpParser {
    fn transform(&mut self, mut packet_info: PacketInfo) -> Result<PacketInfo> {
        let packet_buf = match packet_info.packet {
            SomePacket::UnparsedPacket(buf) => buf,
            _ => panic!(
                "RTCP parser got unexpected packet type: {:x?}",
                packet_info.packet
            ),
        };
        let mut cursor = BitCursor::from_vec(packet_buf);
        let parsed = parse_rtcp_packet(&mut cursor).context("rtcp parse")?;
        packet_info.packet = SomePacket::RtcpPacket(parsed);
        Ok(packet_info)
    }
}

impl From<CompoundRtcpParser> for SomePacketHandler {
    fn from(val: CompoundRtcpParser) -> Self {
        SomePacketHandler::PacketTransformer(Box::new(val))
    }
}
