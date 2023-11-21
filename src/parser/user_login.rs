use tokio::{fs::File, io::AsyncReadExt};

use crate::packet::PrefixedField;

pub(crate) async fn handle_user_login(packet: &[u8]) -> Result<Vec<Vec<u8>>, ()> {
    let parsed_packet = crate::packet::login_request::LoginRequest::from_bytes(&packet);

    debug!("Parsed packet: {:?}", parsed_packet);

    // There are a few steps needed to decrypt the session key and make it usable

    // 1. Start by reading the encrypted session key and displaying it as an ascii string
    let session_key = parsed_packet.get_encrypted_session_key();
    debug!("Encrypted session key: {}", session_key);

    let decrypted_session_key = match decrypt_session_key(session_key).await {
        Ok(value) => value,
        Err(value) => {
            error!("Failed to decrypt session key: {:?}", value);
            return Err(());
        }
    };
    
    // Let's print the decrypted session key as a hex string
    debug!(
        "Decrypted session key: {}",
        hex::encode(&decrypted_session_key)
    );

    let response_message = vec![0x06, 0x01, 0x00, 0x04];

    Ok(vec![response_message])
}

async fn decrypt_session_key(session_key: &str) -> Result<Vec<u8>, ()> {
    let session_key_decode_result = hex::decode(session_key);
    let session_key_bytes = match session_key_decode_result {
        Ok(bytes) => {
            debug!("Session key bytes: {:?}", bytes);
            bytes
        }
        Err(e) => {
            error!("Failed to decode session key: {}", e);
            return Err(());
        }
    };
    if session_key_bytes.len() != 128 {
        error!("Invalid session key length: {}", session_key_bytes.len());
        return Err(());
    }
    let private_key_open_result = File::open("data/private_key.pem").await;
    let mut file = match private_key_open_result {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open private key file: {}", e);
            return Err(());
        }
    };
    let mut private_key_bytes = Vec::new();
    let read_result = file.read_to_end(&mut private_key_bytes).await;
    match read_result {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to read private key file: {}", e);
            return Err(());
        }
    };
    let private_key_create_result = openssl::rsa::Rsa::private_key_from_pem(&private_key_bytes);
    let private_key = match private_key_create_result {
        Ok(key) => key,
        Err(e) => {
            error!("Failed to create private key: {}", e);
            return Err(());
        }
    };
    let mut decrypted_session_key_bytes = vec![0; private_key.size() as usize];
    let key_decrypt_result = private_key.private_decrypt(
        &session_key_bytes,
        &mut decrypted_session_key_bytes,
        openssl::rsa::Padding::PKCS1_OAEP,
    );
    match key_decrypt_result {
        Ok(_) => debug!(
            "Decrypted {} session key bytes",
            decrypted_session_key_bytes.len()
        ),
        Err(e) => {
            error!("Failed to decrypt session key: {}", e);
            return Err(());
        }
    };
    let decrypted_session_key_prefixed_field = PrefixedField::from_bytes(&decrypted_session_key_bytes);
    let decrypted_session_key = decrypted_session_key_prefixed_field.data;
    if decrypted_session_key.len() != 32 {
        error!(
            "Invalid decrypted session key length: {}",
            decrypted_session_key.len()
        );
        return Err(());
    }
    Ok(decrypted_session_key)
}
