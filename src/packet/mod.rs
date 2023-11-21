pub(crate) mod header;
pub(crate) mod login_request;

pub(crate) struct PrefixedField {
    pub(crate) length: u16,
    pub(crate) data: Vec<u8>,
}

impl PrefixedField {
    pub(crate) fn from_bytes(bytes: &[u8]) -> PrefixedField {
        let mut length_bytes = [0; 2];
        length_bytes.copy_from_slice(&bytes[0..2]);
        let length = u16::from_be_bytes(length_bytes);
        debug!("Prefixed field length: {}", length);
        let mut data = vec![0; length as usize];
        data.copy_from_slice(&bytes[2..length as usize + 2]);
        PrefixedField { length, data }
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.data);
        bytes
    }

    pub(crate) fn size(&self) -> usize {
        self.length as usize + 2 // 2 bytes for length
    }
}

impl std::fmt::Debug for PrefixedField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrefixedField")
            .field("length", &self.length)
            .field("data", &hex::encode(&self.data))
            .finish()
    }
}

pub(crate) struct PrefixedString {
    pub(crate) string: String,
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
        debug!("Prefixed with null string length: {}", length);
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

pub(crate) struct MessageContainer {
    id: u16,
    data: Vec<u8>,
}

impl MessageContainer {
    pub(crate) fn from_bytes(bytes: &[u8]) -> MessageContainer {
        let mut id_bytes = [0; 2];
        id_bytes.copy_from_slice(&bytes[0..2]);
        let id = u16::from_be_bytes(id_bytes);
        debug!("Message container id: {}", id);
        let data = bytes[2..].to_vec();
        MessageContainer { id, data }
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id.to_be_bytes());
        bytes.extend_from_slice(&self.data);
        bytes
    }

    pub(crate) fn id(&self) -> u16 {
        self.id
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.data
    }

    pub(crate) fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub(crate) fn size(&self) -> usize {
        self.data.len() + 4 // 4 bytes for header
    }

    pub(crate) fn set_id(&mut self, id: u16) {
        self.id = id;
    }
}

impl std::fmt::Debug for MessageContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageContainer")
            .field("id", &self.id)
            .field("data", &hex::encode(&self.data))
            .finish()
    }
}
