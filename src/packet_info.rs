use rtp_rs::{
    rtcp::rtcp_packet::SomeRtcpPacket,
    rtp::{self, rtp_packet::RtpPacket},
};

#[derive(Debug)]
pub enum SomePacket {
    UnparsedPacket(Vec<u8>),
    UnparsedRtcpPacket(Vec<u8>),
    UnparsedRtpPacket(Vec<u8>),
    RtcpPacket(SomeRtcpPacket),
    RtpPacket(RtpPacket),
}

pub struct PacketInfo {
    pub packet: SomePacket,
}
