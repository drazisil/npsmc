// Desc: Network code

use tokio::io::AsyncReadExt;

use crate::packet::header::Header;
use tokio::macros::support::Future;
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
            let parsed_packet = crate::packet::login_request::LoginRequest::from_bytes(&packet);
            debug!("Loading packet type: {}", packet_type);
            info!("Parsed packet: {:?}", parsed_packet);

            // TODO: Send a response

            // Check if there are any bytes left in the stream
            debug!("Checking for leftover bytes");
            let leftover_bytes = stream.peek(&mut buffer).await.unwrap();
            debug!("Leftover bytes: {}", leftover_bytes);
            return Ok(());
        }
        _ => {
            error!("Unknown packet type: {}", packet_type);
            debug!("Packet: {}", hex::encode(packet));
            return Err(());
        }
    }
}
