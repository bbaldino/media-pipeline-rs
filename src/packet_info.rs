use rtp_rs::rtcp::rtcp_packet::SomeRtcpPacket;

#[derive(Debug)]
pub enum SomePacket {
    UnparsedPacket(Vec<u8>),
    UnparsedRtcpPacket(Vec<u8>),
    UnparsedRtpPacket(Vec<u8>),
    RtcpPacket(SomeRtcpPacket),
    RtpPacket, // dummy for now
}

pub struct PacketInfo {
    pub packet: SomePacket,
}
