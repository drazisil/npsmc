use std::{io::{Read, Write}, net::TcpStream};

use byte_struct::ByteStruct;

use crate::packet::header::Header;

#[derive(Debug)]
enum PacketId {
    LoginRequest = 0x501,
}

impl PacketId {
    fn from_u16(value: u16) -> Option<PacketId> {
        match value {
            0x501 => Some(PacketId::LoginRequest),
            _ => None,
        }
    }
}


// Handle a client connection
pub(crate) fn handle_client(stream: TcpStream, server_name: &str) {
    debug!(
        "New client connection from {} to {}",
        stream.peer_addr().unwrap(),
        server_name
    );
    let mut stream = stream;
    let mut buf = [0; 4];

    // poll for a message
    match stream.read(&mut buf) {
        Ok(_) => {
            // TODO: Parse the header
            let header = Header::read_bytes(&buf);

            // TODO: Get the packet ID
            let packet_id = header.id;
            debug!("Packet ID: {}", packet_id);

            let mut packet_name = "Unknown";

            // Check if the packet ID is valid
            match PacketId::from_u16(packet_id) {
                Some(packet_id) => {
                    debug!("Packet ID: {:?}", packet_id);
                    packet_name = match packet_id {
                        PacketId::LoginRequest => "LoginRequest",
                    };
                }
                None => {
                    error!("Invalid packet ID: {}", packet_id);
                    debug!("Packet name: {}", packet_name);
                    return;
                }
            }

            // TODO: Get the packet length
            let packet_length = header.length;
            debug!("Packet length: {}", packet_length);

            // TODO: Read the packet data
            let mut packet_data = vec![0; packet_length as usize];
            match stream.read(&mut packet_data) {
                Ok(_) => {
                    debug!("Packet data: {}", hex::encode(&packet_data));
                }
                Err(_) => {
                    error!("Error reading packet data from stream");
                }
            }

            // TODO: Parse the packet data

            // TODO: Handle the packet
            debug!("Handling packet {}", packet_name);

            // TODO: Send a response
            let mut response = Header {
                id: packet_id,
                length: packet_length,
            };
            response.id = 0x207;
            response.length = 0x04;

            let response_bytes = response.to_bytes();        

            // Log the response packet as hex
            debug!("Response packet: {}", hex::encode(&response_bytes));

            match stream.write(&response_bytes) {
                Ok(_) => {
                    debug!("Sent response");
                }
                Err(_) => {
                    error!("Error sending response");
                }
            }
        }
        Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
            // do nothing
        }
        Err(_) => {
            error!("Error reading header from stream");
        }
    }
}
