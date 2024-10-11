use std::collections::HashMap;

use anyhow::{bail, Result};
use data_pipeline_rs::data_handler::{DataTransformer, SomeDataHandler};
use rtp_parse::rtcp::rtcp_header;
use webrtc_srtp::{config::Config, context::Context as SrtpContext};

use crate::{
    packet_info::{PacketInfo, SomePacket},
    util::SharedData,
};

pub struct SrtcpDecrypt {
    // TODO: can we do independent contexts for srtp and srtcp? the context type handles both,
    // but it'd be nicer if we didn't have to share...
    // Actually, in practice there should be no contention between this and rtp, since we only
    // process a single packet from a sender at a time, so i should look at just sharing them
    // between here and srtp decrypt
    pub contexts: HashMap<u32, SrtpContext>,
    pub config: SharedData<Config>,
}

impl SrtcpDecrypt {
    fn get_context(&mut self, ssrc: u32) -> &mut SrtpContext {
        self.contexts.entry(ssrc).or_insert_with(|| {
            let config = self.config.read();
            SrtpContext::new(
                &config.keys.local_master_key,
                &config.keys.local_master_salt,
                config.profile,
                // TODO: should pass options in here
                None,
                None,
            )
            .unwrap()
        })
    }
}

impl DataTransformer<PacketInfo> for SrtcpDecrypt {
    fn transform(&mut self, mut data: PacketInfo) -> Result<PacketInfo> {
        match data.packet {
            SomePacket::UnparsedPacket(ref buf) => {
                let ssrc = rtcp_header::get_sender_ssrc(buf);
                let context = self.get_context(ssrc);
                match context.decrypt_rtcp(buf) {
                    Ok(bytes) => {
                        // TODO: we should look at using 'Bytes' everywhere, most likely, but
                        // it's also a bit annoying that webrtc-rs parses the header as part of
                        // the decrypt, using its own types.  need to dig into what to do
                        // there overall.
                        data.packet = SomePacket::UnparsedPacket(bytes.to_vec());
                    }
                    Err(e) => {
                        println!("Error decrypting packet: {e}");
                        bail!("Error decrypting packet: {e}");
                    }
                }
            }
            _ => panic!("Unsupported packet type passed to srtp decrypt"),
        }

        Ok(data)
    }
}

impl From<SrtcpDecrypt> for SomeDataHandler<PacketInfo> {
    fn from(value: SrtcpDecrypt) -> Self {
        SomeDataHandler::Transformer(Box::new(value))
    }
}
