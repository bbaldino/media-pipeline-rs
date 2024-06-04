use compound_rtcp_parser::CompoundRtcpParser;
use demuxer::{ConditionalPath, StaticDemuxer};
use node::{DefaultNode, Node};
use packet_info::SomePacket;
use rtcp_termination::RtcpTermination;
use rtp_parser::RtpParser;
use rtp_rs::util::{looks_like_rtcp, looks_like_rtp};
use tcc_generator::TccGenerator;

pub mod compound_rtcp_parser;
pub mod demuxer;
pub mod node;
pub mod packet_info;
pub mod rtcp_termination;
pub mod rtp_parser;
mod tcc_generator;

pub fn build_pipeline() -> Box<dyn Node> {
    let tcc_generator = Box::new(DefaultNode::from_handler("tcc generator", TccGenerator));
    let mut rtp_parser = Box::new(DefaultNode::from_handler("rtp parser", RtpParser));
    rtp_parser.attach(tcc_generator);
    let mut rtcp_parser = Box::new(DefaultNode::from_handler("rtcp parser", CompoundRtcpParser));

    let rtcp_termination = Box::new(DefaultNode::from_handler(
        "rtcp termination",
        RtcpTermination,
    ));

    rtcp_parser.attach(rtcp_termination);

    let demuxer = StaticDemuxer::new(vec![
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
    ]);

    Box::new(DefaultNode::from_handler("rtp/rtcp demuxer", demuxer))
}

// TODO: wire this up to pcap harness and try it
