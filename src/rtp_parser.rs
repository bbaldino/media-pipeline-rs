use anyhow::{Context, Result};
use rtp_parse::rtp::rtp_packet::read_rtp_packet;

use crate::{
    packet_handler::{PacketTransformer, SomePacketHandler},
    packet_info::{PacketInfo, SomePacket},
    stream_information_store::PayloadTypes,
    util::LiveStateReader,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MediaType {
    Audio,
    Video,
}

pub struct RtpParser {
    payload_types: LiveStateReader<PayloadTypes>,
}

impl RtpParser {
    pub fn new(payload_types: LiveStateReader<PayloadTypes>) -> Self {
        Self { payload_types }
    }
}

impl PacketTransformer for RtpParser {
    fn transform(&mut self, mut packet_info: PacketInfo) -> Result<PacketInfo> {
        match packet_info.packet {
            SomePacket::UnparsedPacket(data) => {
                let rtp_packet = read_rtp_packet(data).context("rtp parse")?;
                // println!("parsed rtp packet: {rtp_packet:?}");
                match self.payload_types.value().get(&rtp_packet.payload_type()) {
                    Some(MediaType::Audio) => {
                        packet_info.packet = SomePacket::AudioRtpPacket(rtp_packet)
                    }
                    Some(MediaType::Video) => {
                        packet_info.packet = SomePacket::VideoRtpPacket(rtp_packet)
                    }
                    None => panic!(
                        "Unable to find media type for payload type {}",
                        rtp_packet.payload_type()
                    ),
                }
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
