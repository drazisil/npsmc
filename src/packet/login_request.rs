use crate::packet::PrefixedStringWithNull;

use super::{header::VersionedHeader, PlainString, PrefixedString};

pub(crate) struct LoginRequest {
    header: VersionedHeader,
    context_id: PrefixedStringWithNull,
    encrypted_session_key: PrefixedString,
    game_id: PrefixedString,
}

impl LoginRequest {
    pub(crate) fn from_bytes(bytes: &[u8]) -> LoginRequest {
        let mut offset = 0;
        let header = VersionedHeader::from_bytes(&bytes[offset..]);
        offset += 12;
        debug!("Loading header: {:?}", header);
        let context_id = PrefixedStringWithNull::from_bytes(&bytes[offset..]);
        offset += context_id.size() + 1;
        debug!("Loading context id: {:?}", context_id);
        let encrypted_session_key = PrefixedString::from_bytes(&bytes[offset..]);
        offset += encrypted_session_key.size();
        debug!("Loading encrypted session key: {:?}", encrypted_session_key);
        let game_id = PrefixedString::from_bytes(&bytes[offset..]);
        debug!("Loading game id: {:?}", game_id);
        LoginRequest {
            header,
            context_id,
            encrypted_session_key,
            game_id,
        }
    }
}

impl std::fmt::Debug for LoginRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoginRequest")
            .field("header", &self.header)
            .field("context_id", &self.context_id.string)
            .field("encrypted_session_key", &self.encrypted_session_key.string)
            .field("game_id", &self.game_id.string)
            .finish()
    }
}
