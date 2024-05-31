use crate::{node::Node, packet_info::PacketInfo};

pub type Predicate<T> = dyn Fn(&T) -> bool;

pub struct ConditionalPath<T, U> {
    pub predicate: Box<Predicate<T>>,
    pub next: U,
}

/// A Demuxer whose paths are fixed and known at creation time, so that they don't need to be
/// locked for each packet.
#[derive(Default)]
pub struct StaticDemuxer {
    packet_paths: Vec<ConditionalPath<PacketInfo, Box<dyn Node>>>,
}

impl StaticDemuxer {
    pub fn new(packet_paths: Vec<ConditionalPath<PacketInfo, Box<dyn Node>>>) -> Self {
        StaticDemuxer { packet_paths }
    }
}

impl Node for StaticDemuxer {
    fn process_packet(&mut self, packet_info: PacketInfo) {
        for path in &mut self.packet_paths {
            if (path.predicate)(&packet_info) {
                path.next.process_packet(packet_info);
                return;
            }
        }
    }

    fn attach(&mut self, next: Box<dyn Node>) {
        panic!("Can't call attach on Demuxer");
    }
}
