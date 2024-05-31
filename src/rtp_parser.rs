use crate::{
    node::{NextNode, Node},
    packet_info::{PacketInfo, SomePacket},
};

#[derive(Default)]
pub struct RtpParser {
    next: NextNode,
}

impl Node for RtpParser {
    fn process_packet(&mut self, mut packet_info: PacketInfo) {
        // TODO: for now just a dummy
        packet_info.packet = SomePacket::RtpPacket;
        self.next.process_packet(packet_info);
    }

    fn attach(&mut self, next: Box<dyn Node>) {
        self.next.replace(next);
    }
}
