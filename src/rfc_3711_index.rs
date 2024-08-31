/// Represents the sequence number used in an RFC3711 Index tracker.
#[derive(Clone, Copy)]
pub struct Rfc3711SeqNum(u16);

impl Rfc3711SeqNum {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    /// Get the difference between self and other, returning a negative value if [`other`] is
    /// larger.  E.g.:
    /// RtpSeqNum(65534).delta_between(RtpSeqNum(65535)) = -1
    ///
    /// * `other`: The other [`RtpSeqNum`] to compare to
    pub fn delta_between(&self, other: &Rfc3711SeqNum) -> i16 {
        ((self.0 as i32) - (other.0 as i32)) as i16
    }

    pub fn is_newer_than(&self, other: &Rfc3711SeqNum) -> bool {
        self.delta_between(other) > 0
    }

    pub fn is_older_than(&self, other: &Rfc3711SeqNum) -> bool {
        self.delta_between(other) < 0
    }

    pub fn rolled_over_to(&self, other: &Rfc3711SeqNum) -> bool {
        // If we're "older" than other, but other is less than we are, then we would've gotten to
        // other by rolling over.
        self.is_older_than(other) && other.0 < self.0
    }

    pub fn as_index(&self, v: u8) -> u32 {
        0x1_0000u32 * v as u32 + self.0 as u32
    }
}

pub struct Rfc3711IndexTracker {
    roc: u8,
    highest_seq_num_seen: Option<Rfc3711SeqNum>,
}

impl Rfc3711IndexTracker {
    pub fn new() -> Self {
        Self {
            roc: 0,
            highest_seq_num_seen: None,
        }
    }

    pub fn update(&mut self, seq_num: u16) -> u32 {
        self.get_index(seq_num, true)
    }

    fn get_index(&mut self, seq_num: u16, update_roc: bool) -> u32 {
        let seq_num_3711 = Rfc3711SeqNum(seq_num);
        let v = if let Some(ref highest_seq_num_seen) = self.highest_seq_num_seen {
            let v = if seq_num_3711.rolled_over_to(highest_seq_num_seen) {
                // This value was from the previous roc value
                self.roc - 1
            } else if highest_seq_num_seen.rolled_over_to(&seq_num_3711) {
                // This sequence number indicates we've rolled over, so update the roc (if
                // update_roc is true) and return the next value
                if update_roc {
                    self.roc += 1;
                    self.roc
                } else {
                    self.roc + 1
                }
            } else {
                self.roc
            };

            if seq_num_3711.is_newer_than(highest_seq_num_seen) {
                self.highest_seq_num_seen = Some(seq_num_3711);
            }
            v
        } else {
            if update_roc {
                self.highest_seq_num_seen = Some(seq_num_3711);
            }
            return seq_num_3711.as_index(0);
        };

        seq_num_3711.as_index(v)
    }
}

impl Default for Rfc3711IndexTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rfc_3711_index() {
        assert_eq!(Rfc3711SeqNum(10).delta_between(&Rfc3711SeqNum(12)), -2);
        assert!(Rfc3711SeqNum(12).is_newer_than(&Rfc3711SeqNum(10)));
        assert!(Rfc3711SeqNum(10).is_older_than(&Rfc3711SeqNum(12)));
        assert!(Rfc3711SeqNum(65535).rolled_over_to(&Rfc3711SeqNum(1)));
        assert_eq!(
            Rfc3711SeqNum(65535).as_index(1),
            0b00000000_00000001_11111111_11111111
        );
        assert_eq!(
            Rfc3711SeqNum(65535).as_index(2),
            0b00000000_00000010_11111111_11111111
        );
    }

    #[test]
    #[allow(clippy::identity_op)]
    fn test_rfc_3711_index_tracker() {
        let mut tracker = Rfc3711IndexTracker::new();
        // Normal
        assert_eq!(tracker.update(65530), 65530);
        // Another, no roll over
        assert_eq!(tracker.update(65531), 65531);
        // Now with roll over
        assert_eq!(tracker.update(2), 1 * 0x1_0000 + 2);
        // Older seq num
        assert_eq!(tracker.update(1), 1 * 0x1_0000 + 1);
        // Older from previous roc
        assert_eq!(tracker.update(65532), 65532);
    }
}
