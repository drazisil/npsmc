// Desc: Network code

use crossterm::terminal::disable_raw_mode;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::packet::header::Header;
use crate::parser::user_login::handle_user_login;
use tokio::net::TcpStream;

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

// Handle a client
pub(crate) async fn handle_client(mut stream: TcpStream, server_name: &str) -> Result<(), ()> {
    let mut buffer = [0; 4];
    let header: Header;
    disable_raw_mode().unwrap();

    // Log the connection
    info!(
        "Connection from {} to {}",
        stream.peer_addr().unwrap(),
        server_name
    );

    // Read the header
    match stream.read_exact(&mut buffer).await {
        Ok(_) => {
            header = Header::from_bytes(&buffer);
            debug!("Loading header: {:?}", header);
        }
        Err(_) => {
            error!("Failed to read header");
            return Err(());
        }
    }

    // Check the header
    if header.id != 0x501 {
        error!("Invalid header id: {}", header.id);
        return Err(());
    }

    // Read the rest of the packet
    let mut packet_buffer = vec![0; header.length as usize - 4];
    match stream.read_exact(&mut packet_buffer).await {
        Ok(_) => {
            debug!("Loading packet: {}", hex::encode(&packet_buffer));
        }
        Err(_) => {
            error!("Failed to read packet");
            return Err(());
        }
    }

    // Combine the header and packet
    let mut packet = header.to_bytes();
    packet.append(&mut packet_buffer);

    let packet_type: &str;

    // Check if thos packet has a known id
    match PacketId::from_u16(header.id) {
        Some(PacketId::LoginRequest) => {
            info!("Login request");
            packet_type = "LoginRequest";
        }
        None => {
            error!("Unknown packet id: {}", header.id);
            debug!("Packet: {}", hex::encode(packet));
            return Err(());
        }
    }

    // If known packet type, load into appropriate struct
    match packet_type {
        "LoginRequest" => {
            let response_packets: Vec<Vec<u8>> = handle_user_login(packet.as_slice()).await?;
            debug!("Loading packet type: {}", packet_type);

            // Send response packets
            for response_packet in response_packets {
                debug!("Sending packet: {}", hex::encode(&response_packet));
                match stream.write_all(&response_packet).await {
                    Ok(_) => {}
                    Err(_) => {
                        error!("Failed to send packet");
                        return Err(());
                    }
                }
            }
        }
        _ => {
            error!("Unknown packet type: {}", packet_type);
            debug!("Packet: {}", hex::encode(packet));
            return Err(());
        }
    }
    Ok(())
}
