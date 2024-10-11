use data_pipeline_rs::data_handler::{DataFilter, SomeDataHandler};
use rtp_parse::rtcp::rtcp_packet::SomeRtcpPacket;

use crate::packet_info::{PacketInfo, SomePacket};

// TODO: we'll have a tokio::sync::broadcast::Sender here to emit events.  its send method isn't
// async so we can call it directly
#[derive(Default)]
pub struct RtcpTermination;

impl DataFilter<PacketInfo> for RtcpTermination {
    fn should_forward(&mut self, data: &PacketInfo) -> bool {
        let rtcp = match &data.packet {
            SomePacket::RtcpPacket(rtcp) => rtcp,
            _ => panic!(
                "RTCP termination received non-rtcp packet {:x?}",
                data.packet
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

impl From<RtcpTermination> for SomeDataHandler<PacketInfo> {
    fn from(value: RtcpTermination) -> Self {
        SomeDataHandler::Filter(Box::new(value))
    }
}
