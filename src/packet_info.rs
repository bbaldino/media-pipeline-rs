use std::{fmt::Display, time::Instant};

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

impl Display for SomePacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SomePacket::VideoRtpPacket(rtp) => write!(f, "{rtp}"),
            _ => write!(f, "some packet"),
        }
    }
}

pub struct PacketInfo {
    pub received_time: Instant,
    pub packet: SomePacket,
    pub should_discard: bool,
}

impl PacketInfo {
    pub fn new(packet: SomePacket, received_time: Instant) -> Self {
        Self {
            received_time,
            packet,
            should_discard: false,
        }
    }

    pub fn new_unparsed(data: Vec<u8>, received_time: Instant) -> Self {
        Self {
            received_time,
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
