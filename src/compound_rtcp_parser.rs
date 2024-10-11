use anyhow::{Context, Result};
use bit_cursor::bit_cursor::BitCursor;
use data_pipeline_rs::data_handler::{DataTransformer, SomeDataHandler};
use rtp_parse::rtcp::rtcp_packet::parse_rtcp_packet;

use crate::packet_info::{PacketInfo, SomePacket};

#[derive(Default)]
pub struct CompoundRtcpParser;

impl DataTransformer<PacketInfo> for CompoundRtcpParser {
    fn transform(&mut self, mut data: PacketInfo) -> Result<PacketInfo> {
        let packet_buf = match data.packet {
            SomePacket::UnparsedPacket(buf) => buf,
            _ => panic!("RTCP parser got unexpected packet type: {:x?}", data.packet),
        };
        let mut cursor = BitCursor::from_vec(packet_buf);
        let parsed = parse_rtcp_packet(&mut cursor).context("rtcp parse")?;
        data.packet = SomePacket::RtcpPacket(parsed);
        Ok(data)
    }
}

impl From<CompoundRtcpParser> for SomeDataHandler<PacketInfo> {
    fn from(val: CompoundRtcpParser) -> Self {
        SomeDataHandler::Transformer(Box::new(val))
    }
}
