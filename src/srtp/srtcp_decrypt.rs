use std::collections::HashMap;

use anyhow::{bail, Result};
use rtp_rs::rtcp::rtcp_header;
pub use webrtc_srtp::{
    config::Config, config::SessionKeys, context::Context as SrtpContext,
    protection_profile::ProtectionProfile,
};

use crate::{
    node::{PacketTransformer, SharedData, SomePacketHandler},
    packet_info::{PacketInfo, SomePacket},
};

pub struct SrtcpDecrypt {
    // TODO: can we do independent contexts for srtp and srtcp? the context type handles both,
    // but it'd be nicer if we didn't have to share...
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

impl PacketTransformer for SrtcpDecrypt {
    fn transform(&mut self, mut packet_info: PacketInfo) -> Result<PacketInfo> {
        match packet_info.packet {
            SomePacket::UnparsedPacket(ref buf) => {
                let ssrc = rtcp_header::get_sender_ssrc(buf);
                let context = self.get_context(ssrc);
                match context.decrypt_rtcp(buf) {
                    Ok(bytes) => {
                        // TODO: we should look at using 'Bytes' everywhere, most likely, but
                        // it's also a bit annoying that webrtc-rs parses the header as part of
                        // the decrypt, using its own types.  need to dig into what to do
                        // there overall.
                        packet_info.packet = SomePacket::UnparsedPacket(bytes.to_vec());
                    }
                    Err(e) => {
                        println!("Error decrypting packet: {e}");
                        bail!("Error decrypting packet: {e}");
                    }
                }
            }
            _ => panic!("Unsupported packet type passed to srtp decrypt"),
        }

        Ok(packet_info)
    }
}

impl From<SrtcpDecrypt> for SomePacketHandler {
    fn from(value: SrtcpDecrypt) -> Self {
        SomePacketHandler::PacketTransformer(Box::new(value))
    }
}
