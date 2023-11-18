use std::{io::Read, net::TcpStream};

pub(crate) struct RawPacket {
    id: u16,       // Packet ID 2 bytes little endian
    length: u16, // Packet length 2 bytes little endian. The length includes the length of the packet ID and length fields
    data: Vec<u8>, // remaining data
}

// Parse a new packet
pub(crate) fn parse_packet(packet: &[u8]) -> RawPacket {
    debug!("Parsing packet");

    let raw_packet = RawPacket {
        id: u16::from_le_bytes([packet[1], packet[0]]),
        length: u16::from_le_bytes([packet[3], packet[2]]),
        data: packet[3..].to_vec(),
    };

    debug!("Packet ID: {}", raw_packet.id);
    debug!("Packet length: {}", raw_packet.length);
    debug!("Packet data: {:X?}", raw_packet.data);
    raw_packet
}

// Handle a client connection
pub(crate) fn handle_client(stream: TcpStream, server_name: &str) {
    debug!(
        "New client connection from {} to {}",
        stream.peer_addr().unwrap(),
        server_name
    );
    let mut stream = stream;
    let mut buf = [0; 1024];

    // poll for a message
    match stream.read(&mut buf) {
        Ok(_) => {
            parse_packet(&buf);
        }
        Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
            // do nothing
        }
        Err(_) => {
            error!("Error reading from stream");
        }
    }
}
