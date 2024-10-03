use std::collections::HashMap;

use crate::{
    packet_handler::{PacketTransformer, SomePacketHandler},
    packet_info::{PacketInfo, SomePacket},
    util::SharedData,
};
use anyhow::{bail, Result};
use rtp_parse::rtp::rtp_header::RtpHeader;
use webrtc_srtp::{config::Config, context::Context as SrtpContext};

// https://datatracker.ietf.org/doc/html/rfc3711#section-3.1
//     0                   1                   2                   3
//    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+<+
//   |V=2|P|X|  CC   |M|     PT      |       sequence number         | |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ |
//   |                           timestamp                           | |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ |
//   |           synchronization source (SSRC) identifier            | |
//   +=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+ |
//   |            contributing source (CSRC) identifiers             | |
//   |                               ....                            | |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ |
//   |                   RTP extension (OPTIONAL)                    | |
// +>+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ |
// | |                          payload  ...                         | |
// | |                               +-------------------------------+ |
// | |                               | RTP padding   | RTP pad count | |
// +>+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+<+
// | ~                     SRTP MKI (OPTIONAL)                       ~ |
// | +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ |
// | :                 authentication tag (RECOMMENDED)              : |
// | +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ |
// |                                                                   |
// +- Encrypted Portion*                      Authenticated Portion ---+

pub struct SrtpDecrypt {
    pub contexts: HashMap<u32, SrtpContext>,
    pub config: SharedData<Config>,
}

impl SrtpDecrypt {
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

impl PacketTransformer for SrtpDecrypt {
    fn transform(&mut self, mut packet_info: PacketInfo) -> Result<PacketInfo> {
        match packet_info.packet {
            SomePacket::UnparsedPacket(ref buf) => {
                let ssrc = RtpHeader::ssrc(buf);
                let context = self.get_context(ssrc);
                match context.decrypt_rtp(buf) {
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

impl From<SrtpDecrypt> for SomePacketHandler {
    fn from(value: SrtpDecrypt) -> Self {
        SomePacketHandler::PacketTransformer(Box::new(value))
    }
}

#[cfg(test)]
mod test {
    use std::{
        io::{Cursor, Read},
        time::Instant,
    };

    use webrtc_srtp::protection_profile::ProtectionProfile;

    use super::*;

    #[test]
    fn test_srtp_decrypt() {
        #[rustfmt::skip]
        let packet: Vec<u8> = vec![
            0x90, 0xEF, 0x43, 0xD7, 0xCF, 0x6F, 0xDE, 0x8F,
            0x56, 0x29, 0x97, 0x7A, 0xBE, 0xDE, 0x00, 0x01,
            0x10, 0xFF, 0x00, 0x00, 0x40, 0xC9, 0xDC, 0x4E,
            0xDD, 0x95, 0x80, 0x41, 0x88, 0x8E, 0xFA, 0x32,
            0x1C, 0x42, 0xB4, 0x03, 0x8B, 0x2D, 0xE5, 0x79,
            0x61, 0xE2, 0x23, 0x7E, 0x17, 0x9C, 0x6E, 0xD7,
            0x6C, 0x6A, 0x11, 0x0D, 0x44, 0x91, 0x33, 0xBE,
            0xE1, 0xD7, 0x0D, 0x41, 0xE4, 0x8B
        ];

        #[rustfmt::skip]
        let expected_decrypted_packet: Vec<u8> = vec![
            0x90, 0xEF, 0x43, 0xD7, 0xCF, 0x6F, 0xDE, 0x8F,
            0x56, 0x29, 0x97, 0x7A, 0xBE, 0xDE, 0x00, 0x01,
            0x10, 0xFF, 0x00, 0x00, 0x78, 0x0B, 0xE4, 0xC1,
            0x36, 0xEC, 0xC5, 0x8D, 0x8C, 0x49, 0x46, 0x99,
            0x04, 0xC5, 0xAA, 0xED, 0x92, 0xE7, 0x63, 0x4A,
            0x3A, 0x18, 0x98, 0xEE, 0x62, 0xCB, 0x60, 0xFF,
            0x6C, 0x1B, 0x29, 0x00
        ];

        #[rustfmt::skip]
        let keying_material: Vec<u8> = vec![
            0xB4, 0x04, 0x3B, 0x87, 0x67, 0xF6, 0xC4, 0x67,
            0xB2, 0x3E, 0xE1, 0xBE, 0x0C, 0xEB, 0x8E, 0x24,
            0xA0, 0x4F, 0xA4, 0x36, 0xC4, 0x17, 0x87, 0xF5,
            0xF5, 0x0C, 0xE4, 0x1A, 0x39, 0xFC, 0xB8, 0x21,
            0xDD, 0xC5, 0x60, 0x46, 0xCE, 0x69, 0x63, 0x55,
            0x8E, 0xF1, 0x9A, 0x35, 0x73, 0x0A, 0x4B, 0x69,
            0x17, 0x80, 0xD8, 0x96, 0x19, 0x85, 0xD0, 0xEF,
            0x32, 0x00, 0xCC, 0x27
        ];
        let ssrc = 0x5629977a;
        let key_len = ProtectionProfile::Aes128CmHmacSha1_80.key_len();
        let salt_len = ProtectionProfile::Aes128CmHmacSha1_80.salt_len();
        let mut keying_material_cursor = Cursor::new(keying_material);
        let mut client_write_master_key: Vec<u8> = vec![0u8; key_len];
        let mut server_write_master_key: Vec<u8> = vec![0u8; key_len];
        let mut client_write_master_salt: Vec<u8> = vec![0u8; salt_len];
        let mut server_write_master_salt: Vec<u8> = vec![0u8; salt_len];
        keying_material_cursor
            .read_exact(&mut client_write_master_key)
            .unwrap();
        keying_material_cursor
            .read_exact(&mut server_write_master_key)
            .unwrap();
        keying_material_cursor
            .read_exact(&mut client_write_master_salt)
            .unwrap();
        keying_material_cursor
            .read_exact(&mut server_write_master_salt)
            .unwrap();

        let context = SrtpContext::new(
            &server_write_master_key,
            &server_write_master_salt,
            ProtectionProfile::Aes128CmHmacSha1_80,
            None,
            None,
        )
        .unwrap();

        // We won't actually use this here, so just pass default in for now
        let config = SharedData::new(Config::default());

        let mut transformer = SrtpDecrypt {
            contexts: HashMap::from([(ssrc, context)]),
            config,
        };

        let result = transformer
            .transform(PacketInfo {
                packet: SomePacket::UnparsedPacket(packet),
                should_discard: false,
                received_time: Instant::now(),
            })
            .unwrap();

        match result.packet {
            SomePacket::UnparsedPacket(data) => {
                assert_eq!(data, expected_decrypted_packet);
            }
            _ => panic!("wrong output"),
        }
    }
}
