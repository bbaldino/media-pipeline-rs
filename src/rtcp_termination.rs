use rtp_rs::rtcp::rtcp_packet::SomeRtcpPacket;

use crate::{
    node::{NextNode, Node},
    packet_info::{PacketInfo, SomePacket},
};

// TODO: we'll have a tokio::sync::broadcast::Sender here to emit events.  its send method isn't
// async so we can call it directly
#[derive(Default)]
pub struct RtcpTermination {
    next: NextNode,
}

impl Node for RtcpTermination {
    fn process_packet(&mut self, packet_info: PacketInfo) {
        let rtcp = match packet_info.packet {
            SomePacket::RtcpPacket(rtcp) => rtcp,
            _ => {
                println!(
                    "RTCP pipeline received non-RTCP packet {:?}",
                    packet_info.packet
                );
                return;
            }
        };
        match rtcp {
            SomeRtcpPacket::CompoundRtcpPacket(_) => {
                println!("got compound rtcp");
            }
            SomeRtcpPacket::RtcpRrPacket(_) => {
                println!("got rr");
            }
            SomeRtcpPacket::RtcpByePacket(_) => println!("got bye"),
            SomeRtcpPacket::RtcpSrPacket(_) => println!("got sr"),
            SomeRtcpPacket::RtcpSdesPacket(_) => println!("got sdes"),
            SomeRtcpPacket::RtcpFbNackPacket(_) => println!("got nack"),
            SomeRtcpPacket::RtcpFbFirPacket(_) => println!("got fir"),
            SomeRtcpPacket::RtcpFbTccPacket(_) => println!("got tcc"),
            SomeRtcpPacket::UnknownRtcpPacket { header, payload } => println!("got unknown"),
        }
    }

    fn attach(&mut self, next: Box<dyn Node>) {
        self.next.replace(next);
    }
}
