use crate::{
    node::{Node, PacketDemuxer, SomePacketHandler},
    packet_info::{PacketInfo, SomePacket},
};

pub struct AvDemuxer {
    audio_path: Box<dyn Node>,
    video_path: Box<dyn Node>,
}

impl AvDemuxer {
    pub fn new(audio_path: Box<dyn Node>, video_path: Box<dyn Node>) -> Self {
        Self {
            audio_path,
            video_path,
        }
    }
}

impl PacketDemuxer for AvDemuxer {
    fn find_path(&mut self, packet_info: &PacketInfo) -> Option<&mut dyn Node> {
        if matches!(packet_info.packet, SomePacket::AudioRtpPacket(_)) {
            return Some(&mut *self.audio_path);
        } else if matches!(packet_info.packet, SomePacket::VideoRtpPacket(_)) {
            return Some(&mut *self.video_path);
        }
        None
    }

    fn visit(&mut self, visitor: &mut dyn crate::node::NodeVisitor) {
        self.audio_path.visit(visitor);
        self.video_path.visit(visitor);
    }
}

impl From<AvDemuxer> for SomePacketHandler {
    fn from(value: AvDemuxer) -> Self {
        SomePacketHandler::PacketDemuxer(Box::new(value))
    }
}
