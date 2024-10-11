use data_pipeline_rs::{
    data_handler::{DataDemuxer, SomeDataHandler},
    node::NodeRef,
    node_visitor::NodeVisitor,
};

use crate::packet_info::{PacketInfo, SomePacket};

pub struct AvDemuxer {
    audio_path: NodeRef<PacketInfo>,
    video_path: NodeRef<PacketInfo>,
}

impl AvDemuxer {
    pub fn new(audio_path: NodeRef<PacketInfo>, video_path: NodeRef<PacketInfo>) -> Self {
        Self {
            audio_path,
            video_path,
        }
    }
}

impl DataDemuxer<PacketInfo> for AvDemuxer {
    fn find_path(&mut self, data: &PacketInfo) -> Option<&NodeRef<PacketInfo>> {
        if matches!(data.packet, SomePacket::AudioRtpPacket(_)) {
            return Some(&self.audio_path);
        } else if matches!(data.packet, SomePacket::VideoRtpPacket(_)) {
            return Some(&self.video_path);
        }
        None
    }

    fn visit(&mut self, visitor: &mut dyn NodeVisitor<PacketInfo>) {
        self.audio_path.visit(visitor);
        self.video_path.visit(visitor);
    }
}

impl From<AvDemuxer> for SomeDataHandler<PacketInfo> {
    fn from(value: AvDemuxer) -> Self {
        SomeDataHandler::Demuxer(Box::new(value))
    }
}
