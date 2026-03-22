use crate::nfsv4::types::{ClientId4, SequenceId4, SessionId4, SlotId4};

/// Session/bootstrap state for NFSv4.1.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SessionState {
    pub clientid: Option<ClientId4>,
    pub sessionid: Option<SessionId4>,
    pub sequenceid: SequenceId4,
    pub slotid: SlotId4,
    pub highest_slotid: SlotId4,
}

impl SessionState {
    pub fn next_sequence(&mut self) -> u32 {
        let seq = self.sequenceid;
        self.sequenceid += 1;
        seq
    }

    pub fn is_initialized(&self) -> bool {
        self.clientid.is_some() && self.sessionid.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sessionstate_default() {
        let s = SessionState::default();

        assert_eq!(s.clientid, None);
        assert_eq!(s.sessionid, None);
        assert_eq!(s.sequenceid, 0);
        assert_eq!(s.slotid, 0);
        assert_eq!(s.highest_slotid, 0);
        assert!(!s.is_initialized());
    }

    #[test]
    fn test_sessionstate_is_initialized() {
        let mut s = SessionState::default();

        // まだ未初期化
        assert!(!s.is_initialized());

        s.clientid = Some(1);
        assert!(!s.is_initialized());

        s.sessionid = Some([0u8; 16]);
        assert!(s.is_initialized());
    }

    #[test]
    fn test_next_sequence_increments() {
        let mut s = SessionState::default();

        assert_eq!(s.next_sequence(), 0);
        assert_eq!(s.sequenceid, 1);

        assert_eq!(s.next_sequence(), 1);
        assert_eq!(s.sequenceid, 2);

        assert_eq!(s.next_sequence(), 2);
        assert_eq!(s.sequenceid, 3);
    }

    #[test]
    fn test_next_sequence_with_initial_value() {
        let mut s = SessionState {
            sequenceid: 10,
            ..Default::default()
        };

        assert_eq!(s.next_sequence(), 10);
        assert_eq!(s.sequenceid, 11);
    }
}
