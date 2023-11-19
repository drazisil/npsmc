pub(crate) mod header;
pub(crate) mod login_request;

pub(crate) struct PrefixedString {
    string: String,
}

impl PrefixedString {
    pub(crate) fn from_bytes(bytes: &[u8]) -> PrefixedString {
        let mut length_bytes = [0; 2];
        length_bytes.copy_from_slice(&bytes[0..2]);
        let length = u16::from_be_bytes(length_bytes);
        debug!("Prefixed string length: {}", length);
        let string = String::from_utf8(bytes[2..length as usize + 2].to_vec()).unwrap();
        PrefixedString { string }
    }

    pub(crate) fn size(&self) -> usize {
        self.string.len() + 2 // 2 bytes for length
    }
}

impl std::fmt::Debug for PrefixedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrefixedString")
            .field("string", &self.string)
            .finish()
    }
}

pub(crate) struct PrefixedStringWithNull {
    string: String,
}

impl PrefixedStringWithNull {
    pub(crate) fn from_bytes(bytes: &[u8]) -> PrefixedStringWithNull {
        let mut length_bytes = [0; 2];
        length_bytes.copy_from_slice(&bytes[0..2]);
        let length = u16::from_be_bytes(length_bytes);
        debug!("Prefixed string length: {}", length);
        let string = String::from_utf8(bytes[2..length as usize + 2].to_vec()).unwrap();
        PrefixedStringWithNull { string }
    }

    pub(crate) fn size(&self) -> usize {
        self.string.len() + 2 + 1 // 2 bytes for length, 1 byte for null terminator
    }
}

impl std::fmt::Debug for PrefixedStringWithNull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrefixedStringWithNull")
            .field("string", &self.string)
            .finish()
    }
}

pub(crate) struct PlainString {
    string: String,
}

impl PlainString {
    pub(crate) fn from_bytes(bytes: &[u8], length: usize) -> PlainString {
        let string = String::from_utf8(bytes[0..length].to_vec()).unwrap();
        PlainString { string }
    }
}

impl std::fmt::Debug for PlainString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlainString")
            .field("string", &self.string)
            .finish()
    }
}
