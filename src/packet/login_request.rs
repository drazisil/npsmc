use crate::packet::PrefixedStringWithNull;

use super::{header::VersionedHeader, PlainString, PrefixedString};

pub(crate) struct LoginRequest {
    header: VersionedHeader,
    context_id: PrefixedString,
    encrypted_session_key: PrefixedString,
    game_id: PrefixedString,
}

impl LoginRequest {
    pub(crate) fn from_bytes(bytes: &[u8]) -> LoginRequest {
        let mut offset = 0;
        let header = VersionedHeader::from_bytes(&bytes[offset..]);
        offset += 12;
        debug!("Loading header: {:?}", header);
        let context_id = PrefixedString::from_bytes(&bytes[offset..]);
        offset += context_id.size();
        debug!("Loading context id: {:?}", context_id);
        // The next part is a MessageContainer with a id set to 0
        let message_container = super::MessageContainer::from_bytes(&bytes[offset..]);

        // Skip the empty id, reset the offset to the start of the data
        let mut offset = 0;
        let rest_of_message = message_container.data();

        let encrypted_session_key = PrefixedString::from_bytes(&rest_of_message[offset..]);
        offset += encrypted_session_key.size();
        debug!("Loading encrypted session key: {:?}", encrypted_session_key);
        let game_id = PrefixedString::from_bytes(&rest_of_message[offset..]);
        debug!("Loading game id: {:?}", game_id);
        LoginRequest {
            header,
            context_id,
            encrypted_session_key,
            game_id,
        }
    }

    pub(crate) fn get_encrypted_session_key(&self) -> &str {
        &self.encrypted_session_key.string as &str
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
