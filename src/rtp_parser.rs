use anyhow::{Context, Result};
use data_pipeline_rs::data_handler::{DataTransformer, SomeDataHandler};
use rtp_parse::rtp::rtp_packet::read_rtp_packet;

use crate::{
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

impl DataTransformer<PacketInfo> for RtpParser {
    fn transform(&mut self, mut data: PacketInfo) -> Result<PacketInfo> {
        match data.packet {
            SomePacket::UnparsedPacket(packet_data) => {
                let rtp_packet = read_rtp_packet(packet_data).context("rtp parse")?;
                // println!("parsed rtp packet: {rtp_packet:?}");
                match self.payload_types.value().get(&rtp_packet.payload_type()) {
                    Some(MediaType::Audio) => data.packet = SomePacket::AudioRtpPacket(rtp_packet),
                    Some(MediaType::Video) => data.packet = SomePacket::VideoRtpPacket(rtp_packet),
                    None => panic!(
                        "Unable to find media type for payload type {}",
                        rtp_packet.payload_type()
                    ),
                }
                Ok(data)
            }
            _ => panic!("RTP parser got unexpected packet type: {:x?}", data.packet),
        }
    }
}

impl From<RtpParser> for SomeDataHandler<PacketInfo> {
    fn from(val: RtpParser) -> Self {
        SomeDataHandler::Transformer(Box::new(val))
    }
}
