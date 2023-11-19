use byte_struct::*;

#[derive(ByteStruct, Debug)]
#[byte_struct_be]
pub(crate) struct Header {
    pub id: u16,
    pub length: u16,
}
impl Header {
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id.to_be_bytes());
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes
    }
}
