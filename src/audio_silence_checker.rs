use anyhow::Result;
use rtp_rs::rtp::audio_level_header_extension::is_muted;

use crate::{
    node::{PacketTransformer, SomePacketHandler},
    packet_info::{PacketInfo, SomePacket},
};

pub struct AudioSilenceChecker;

impl PacketTransformer for AudioSilenceChecker {
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

impl From<AudioSilenceChecker> for SomePacketHandler {
    fn from(value: AudioSilenceChecker) -> Self {
        SomePacketHandler::PacketTransformer(Box::new(value))
    }
}
