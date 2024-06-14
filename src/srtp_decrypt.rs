use std::collections::HashMap;

use crate::{node::PacketTransformer, srtp::srtp_crypto_context::SrtpCryptoContext};

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

struct SrtpDecrypt {
    contexts: HashMap<u32, SrtpCryptoContext>,
}

impl PacketTransformer for SrtpDecrypt {
    fn transform(
        &mut self,
        packet_info: crate::packet_info::PacketInfo,
    ) -> anyhow::Result<crate::packet_info::PacketInfo> {
        todo!()
    }
}
