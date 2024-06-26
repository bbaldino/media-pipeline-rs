use rtp_rs::{rtcp::rtcp_packet::SomeRtcpPacket, rtp::rtp_packet::RtpPacket};

#[derive(Debug)]
pub enum SomePacket {
    UnparsedPacket(Vec<u8>),
    UnparsedRtcpPacket(Vec<u8>),
    UnparsedRtpPacket(Vec<u8>),
    RtcpPacket(SomeRtcpPacket),
    RtpPacket(RtpPacket),
    AudioRtpPacket(RtpPacket),
    VideoRtpPacket(RtpPacket),
}

pub struct PacketInfo {
    pub packet: SomePacket,
    pub should_discard: bool,
}

impl PacketInfo {
    pub fn new(packet: SomePacket) -> Self {
        Self {
            packet,
            should_discard: false,
        }
    }

    pub fn new_unparsed(data: Vec<u8>) -> Self {
        Self {
            packet: SomePacket::UnparsedPacket(data),
            should_discard: false,
        }
    }
}

pub fn looks_like_rtp(packet_info: &PacketInfo) -> bool {
    match &packet_info.packet {
        SomePacket::UnparsedPacket(ref packet) => rtp_rs::util::looks_like_rtp(packet),
        some_packet => panic!("Unexpected packet type: {some_packet:?}"),
    }
}

pub fn looks_like_rtcp(packet_info: &PacketInfo) -> bool {
    match &packet_info.packet {
        SomePacket::UnparsedPacket(ref packet) => rtp_rs::util::looks_like_rtcp(packet),
        some_packet => panic!("Unexpected packet type: {some_packet:?}"),
    }
}
