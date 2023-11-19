use byte_struct::*;

#[derive(ByteStruct, Debug)]
#[byte_struct_be]
pub struct Header {
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

    pub(crate) fn from_bytes(bytes: &[u8]) -> Header {
        let mut id_bytes = [0; 2];
        id_bytes.copy_from_slice(&bytes[0..2]);
        let mut length_bytes = [0; 2];
        length_bytes.copy_from_slice(&bytes[2..4]);
        Header {
            id: u16::from_be_bytes(id_bytes),
            length: u16::from_be_bytes(length_bytes),
        }
    }
}

#[derive(ByteStruct, Debug)]
#[byte_struct_be]
pub struct VersionedHeader {
    pub id: u16,
    pub length: u16,
    version: u16,
    reserved: u16,
    checksum: u32,
}

impl VersionedHeader {


    pub(crate) fn from_bytes(bytes: &[u8]) -> VersionedHeader {
        let mut id_bytes = [0; 2];
        id_bytes.copy_from_slice(&bytes[0..2]);
        let mut length_bytes = [0; 2];
        length_bytes.copy_from_slice(&bytes[2..4]);
        let mut version_bytes = [0; 2];
        version_bytes.copy_from_slice(&bytes[4..6]);
        let mut reserved_bytes = [0; 2];
        reserved_bytes.copy_from_slice(&bytes[6..8]);
        let mut checksum_bytes = [0; 4];
        checksum_bytes.copy_from_slice(&bytes[8..12]);
        VersionedHeader {
            id: u16::from_be_bytes(id_bytes),
            length: u16::from_be_bytes(length_bytes),
            version: u16::from_be_bytes(version_bytes),
            reserved: u16::from_be_bytes(reserved_bytes),
            checksum: u32::from_be_bytes(checksum_bytes),
        }
    }
}