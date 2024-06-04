use std::collections::HashMap;

use anyhow::{Context, Result};
use bitcursor::ux::u7;
use rtp_rs::rtp::rtp_packet::read_rtp_packet;

use crate::{
    node::{PacketTransformer, SharedData, SomePacketHandler},
    packet_info::{PacketInfo, SomePacket},
};

pub enum MediaType {
    Audio,
    Video,
}

#[derive(Default)]
pub struct StreamInformationStore {
    pub pt_map: HashMap<u7, MediaType>,
}

pub struct RtpParser {
    stream_information: SharedData<StreamInformationStore>,
}

impl RtpParser {
    pub fn new(stream_information: SharedData<StreamInformationStore>) -> Self {
        Self { stream_information }
    }
}

impl PacketTransformer for RtpParser {
    fn transform(&mut self, mut packet_info: PacketInfo) -> Result<PacketInfo> {
        match packet_info.packet {
            SomePacket::UnparsedPacket(data) => {
                let rtp_packet = read_rtp_packet(data).context("rtp parse")?;
                match self
                    .stream_information
                    .read()
                    .pt_map
                    .get(&rtp_packet.header.payload_type)
                {
                    Some(MediaType::Audio) => {
                        packet_info.packet = SomePacket::AudioRtpPacket(rtp_packet)
                    }
                    Some(MediaType::Video) => {
                        packet_info.packet = SomePacket::VideoRtpPacket(rtp_packet)
                    }
                    None => panic!(
                        "Unable to find media type for payload type {}",
                        rtp_packet.header.payload_type
                    ),
                }
                // println!("parsed rtp packet {rtp_packet:x?}");
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
