use compound_rtcp_parser::CompoundRtcpParser;
use demuxer::{ConditionalPath, StaticDemuxer};
use node::Node;
use packet_info::SomePacket;
use rtcp_termination::RtcpTermination;
use rtp_parser::RtpParser;
use rtp_rs::util::{looks_like_rtcp, looks_like_rtp};

pub mod compound_rtcp_parser;
pub mod demuxer;
pub mod node;
pub mod packet_info;
pub mod rtcp_termination;
pub mod rtp_parser;

pub fn build_pipeline() -> Box<dyn Node> {
    let rtp_parser = Box::<RtpParser>::default();
    let mut rtcp_parser = Box::<CompoundRtcpParser>::default();

    let rtcp_termination = Box::<RtcpTermination>::default();

    rtcp_parser.attach(rtcp_termination);

    Box::new(StaticDemuxer::new(vec![
        ConditionalPath {
            predicate: Box::new(|packet_info| match packet_info.packet {
                SomePacket::UnparsedPacket(ref buf) => looks_like_rtp(&buf[..]),
                _ => {
                    println!("rtp/rtcp demuxer received data other than UnparsedPacket");
                    false
                }
            }),
            next: rtp_parser,
        },
        ConditionalPath {
            predicate: Box::new(|packet_info| match packet_info.packet {
                SomePacket::UnparsedPacket(ref buf) => looks_like_rtcp(&buf[..]),
                _ => {
                    println!("rtp/rtcp demuxer received data other than UnparsedPacket");
                    false
                }
            }),
            next: rtcp_parser,
        },
    ]))
}

// TODO: wire this up to pcap harness and try it
