use anyhow::Result;
use data_pipeline::data_handler::{DataTransformer, SomeDataHandler};
use rtp_parse::rtp::audio_level_header_extension::is_muted;

use crate::packet_info::{PacketInfo, SomePacket};

pub struct AudioSilenceChecker;

impl DataTransformer<PacketInfo> for AudioSilenceChecker {
    fn transform(&mut self, mut packet_info: PacketInfo) -> Result<PacketInfo> {
        let rtp_packet = match packet_info.packet {
            SomePacket::AudioRtpPacket(ref rtp) => rtp,
            _ => panic!(
                "AudioLevelReader got non-rtp packet: {:?}",
                packet_info.packet
            ),
        };
        if let Some(audio_level_ext) = rtp_packet.get_extension_by_id(1) {
            if is_muted(audio_level_ext) {
                packet_info.should_discard = true;
            }
        }

        Ok(packet_info)
    }
}

impl From<AudioSilenceChecker> for SomeDataHandler<PacketInfo> {
    fn from(value: AudioSilenceChecker) -> Self {
        SomeDataHandler::Transformer(Box::new(value))
    }
}
