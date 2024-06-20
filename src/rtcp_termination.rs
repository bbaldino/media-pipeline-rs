use rtp_rs::rtcp::rtcp_packet::SomeRtcpPacket;

use crate::{
    packet_handler::{PacketFilter, SomePacketHandler},
    packet_info::{PacketInfo, SomePacket},
};

// TODO: we'll have a tokio::sync::broadcast::Sender here to emit events.  its send method isn't
// async so we can call it directly
#[derive(Default)]
pub struct RtcpTermination;

impl PacketFilter for RtcpTermination {
    fn should_forward(&mut self, packet_info: &PacketInfo) -> bool {
        let rtcp = match &packet_info.packet {
            SomePacket::RtcpPacket(rtcp) => rtcp,
            _ => panic!(
                "RTCP termination received non-rtcp packet {:x?}",
                packet_info.packet
            ),
        };
        match rtcp {
            SomeRtcpPacket::CompoundRtcpPacket(packets) => {
                println!("got compound rtcp");
                for packet in packets {
                    match packet {
                        SomeRtcpPacket::RtcpRrPacket(_) => println!("got rr"),
                        SomeRtcpPacket::RtcpByePacket(_) => println!("got bye"),
                        SomeRtcpPacket::RtcpSrPacket(_) => println!("got sr"),
                        SomeRtcpPacket::RtcpSdesPacket(_) => println!("got sdes"),
                        SomeRtcpPacket::RtcpFbNackPacket(_) => println!("got nack"),
                        SomeRtcpPacket::RtcpFbFirPacket(_) => println!("got fir"),
                        SomeRtcpPacket::RtcpFbTccPacket(_) => println!("got tcc"),
                        SomeRtcpPacket::RtcpFbPliPacket(_) => println!("got pli"),
                        SomeRtcpPacket::UnknownRtcpPacket { .. } => println!("got unknown"),
                        SomeRtcpPacket::CompoundRtcpPacket(_) => {
                            panic!("compound inside compound is invalid")
                        }
                    }
                }
            }
            SomeRtcpPacket::RtcpRrPacket(_) => println!("got rr"),
            SomeRtcpPacket::RtcpByePacket(_) => println!("got bye"),
            SomeRtcpPacket::RtcpSrPacket(_) => println!("got sr"),
            SomeRtcpPacket::RtcpSdesPacket(_) => println!("got sdes"),
            SomeRtcpPacket::RtcpFbNackPacket(_) => println!("got nack"),
            SomeRtcpPacket::RtcpFbFirPacket(_) => println!("got fir"),
            SomeRtcpPacket::RtcpFbTccPacket(_) => println!("got tcc"),
            SomeRtcpPacket::RtcpFbPliPacket(_) => println!("got pli"),
            SomeRtcpPacket::UnknownRtcpPacket { .. } => println!("got unknown"),
        };
        false
    }
}

impl From<RtcpTermination> for SomePacketHandler {
    fn from(value: RtcpTermination) -> Self {
        SomePacketHandler::PacketFilter(Box::new(value))
    }
}
