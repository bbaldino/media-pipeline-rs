use anyhow::{Context, Result};
use rtp_rs::rtp::rtp_packet::read_rtp_packet;

use crate::{
    node::{PacketTransformer, SomePacketHandler},
    packet_info::{PacketInfo, SomePacket},
};

#[derive(Default)]
pub struct RtpParser;

impl PacketTransformer for RtpParser {
    fn transform(&mut self, mut packet_info: PacketInfo) -> Result<PacketInfo> {
        match packet_info.packet {
            SomePacket::UnparsedPacket(data) => {
                let rtp_packet = read_rtp_packet(data).context("rtp parse")?;
                // println!("parsed rtp packet {rtp_packet:x?}");
                packet_info.packet = SomePacket::RtpPacket(rtp_packet);
                Ok(packet_info)
            }
            _ => panic!(
                "RTP parser got unexpected packet type: {:x?}",
                packet_info.packet
            ),
        }
    }
}

impl From<RtpParser> for SomePacketHandler {
    fn from(val: RtpParser) -> Self {
        SomePacketHandler::PacketTransformer(Box::new(val))
    }
}
