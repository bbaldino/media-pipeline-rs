use bitcursor::ux::u48;

/// https://datatracker.ietf.org/doc/html/rfc3711#section-3.2.1
pub struct SrtpCryptoContext {
    /// a 32-bit unsigned rollover counter (ROC), which records how many
    ///   times the 16-bit RTP sequence number has been reset to zero after
    ///   passing through 65,535.  Unlike the sequence number (SEQ), which
    ///   SRTP extracts from the RTP packet header, the ROC is maintained by
    ///   SRTP as described in Section 3.3.1.
    ///
    /// We define the index of the SRTP packet corresponding to a given
    ///   ROC and RTP sequence number to be the 48-bit quantity
    ///               i = 2^16 * ROC + SEQ.
    pub roc: u32,

    /// for the receiver only, a 16-bit sequence number s_l, which can be
    ///   thought of as the highest received RTP sequence number (see
    ///   Section 3.3.1 for its handling), which SHOULD be authenticated
    ///   since message authentication is RECOMMENDED,
    pub s_l: Option<u16>,

    pub master_key: Vec<u8>,
    pub master_salt: Vec<u8>,
}

impl SrtpCryptoContext {}

pub fn get_srtp_index(roc: u32, seq_num: u16) -> u48 {
    u48::new(((roc as u64) << 16) + seq_num as u64)
}
