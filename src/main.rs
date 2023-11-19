use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tokio::net::TcpListener;
#[macro_use]
extern crate log as log_crate;
extern crate simplelog;

use crate::{log::init_logging, net::handle_client};

mod log;
mod net;
mod packet;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let login_port = 8226;
    let persona_port = 8228;
    let lobby_port = 7003;

    init_logging();

    println!("Starting server");
    println!("Press ? for help");
    println!("Press x to quit");

    let login_listener = TcpListener::bind(("0.0.0.0", login_port)).await?;
    let persona_listener = TcpListener::bind(("0.0.0.0", persona_port)).await?;
    let lobby_listener = TcpListener::bind(("0.0.0.0", lobby_port)).await?;

    // Main loop
    loop {
        // Set stdin to non-blocking
        match enable_raw_mode() {
            Ok(_) => {}
            Err(_) => {
                error!("Failed to set stdin to non-blocking");
                break;
            }
        }

        // Poll stdin for input
        if crossterm::event::poll(std::time::Duration::from_millis(100)).unwrap() {
            match crossterm::event::read().unwrap() {
                crossterm::event::Event::Key(key) => match key.code {
                    crossterm::event::KeyCode::Char('x') => {
                        disable_raw_mode().unwrap();
                        println!("Quitting");
                        break;
                    }
                    crossterm::event::KeyCode::Char('?') => {
                        disable_raw_mode().unwrap();
                        println!("You can press x to quit");
                        break;
                    }
                    _ => {
                        // Cancel out the key press
                        continue;
                    }
                },
                _ => {}
            }
        }

        // Set stdin back to blocking
        match disable_raw_mode() {
            Ok(_) => {}
            Err(_) => {
                error!("Failed to set stdin back to blocking");
                break;
            }
        }

        // Check for incoming connections
        let login_result = login_listener.accept().await;
        if login_result.is_ok() {
            debug!("Login connection");
            let (socket, _) = login_result.unwrap();
            tokio::spawn(handle_client(socket, "login"));
        }

        let persona_result = persona_listener.accept().await;
        if persona_result.is_ok() {
            debug!("Persona connection");
            let (socket, _) = persona_result.unwrap();
            tokio::spawn(handle_client(socket, "persona"));
        }

        let lobby_result = lobby_listener.accept().await;
        if lobby_result.is_ok() {
            debug!("Lobby connection");
            let (socket, _) = lobby_result.unwrap();
            tokio::spawn(handle_client(socket, "lobby"));            
        }
    }

    println!("Server shutting down");

    Ok(())
}
