use std::borrow::BorrowMut;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tokio::net::TcpListener;
use tokio::sync::watch;
#[macro_use]
extern crate log as log_crate;
extern crate simplelog;

use crate::{log::init_logging, net::handle_client};

mod log;
mod net;
mod packet;

fn print_help() {
    println!("Help:");
    println!("? - Print this help");
    println!("x - Quit");
}

enum Keys {
    None,
    Quit,
}

fn check_for_key() -> Keys {
    // Set stdin to non-blocking
    match enable_raw_mode() {
        Ok(_) => {}
        Err(_) => {
            error!("Failed to set stdin to non-blocking");
        }
    }

    // Poll stdin for input
    if crossterm::event::poll(std::time::Duration::from_millis(100)).unwrap() {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key) => match key.code {
                crossterm::event::KeyCode::Char('x') => {
                    disable_raw_mode().unwrap();
                    println!("Quitting");
                    return Keys::Quit;
                }
                crossterm::event::KeyCode::Char('?') => {
                    disable_raw_mode().unwrap();
                    print_help();
                }
                _ => {
                    disable_raw_mode().unwrap();
                    // Swallow the key
                    return Keys::None;
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
        }
    }
    return Keys::None;
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let login_port = 8226;
    let persona_port = 8228;
    let lobby_port = 7003;

    init_logging();

    println!("Welcome to the Rusty Motors Server");

    // Print help
    print_help();

    let login_listener = TcpListener::bind(("0.0.0.0", login_port)).await?;
    debug!("Listening on port {}", login_port);
    let persona_listener = TcpListener::bind(("0.0.0.0", persona_port)).await?;
    debug!("Listening on port {}", persona_port);
    let lobby_listener = TcpListener::bind(("0.0.0.0", lobby_port)).await?;
    debug!("Listening on port {}", lobby_port);

    let (tx, rx) = watch::channel(true);

    let login_rx = rx.clone();
    let persona_rx = rx.clone();
    let lobby_rx = rx.clone();

    // Spawn listeners
    tokio::spawn(async move {
        loop {
            if !*login_rx.borrow() {
                debug!("Login listener shutting down");
                break;
            }

            let login_result = login_listener.accept().await;
            if let Err(e) = login_result {
                error!("Failed to accept login connection: {}", e);
                continue;
            }
            if let Ok((socket, _)) = login_result {
                debug!("Login connection");
                tokio::spawn(handle_client(socket, "login"));
            }
        }
    });

    let persona_handle = tokio::spawn(async move {
        loop {
            if !*persona_rx.borrow() {
                debug!("Persona listener shutting down");
                break;
            }

            let persona_result = persona_listener.accept().await;
            if let Err(e) = persona_result {
                error!("Failed to accept persona connection: {}", e);
                continue;
            }
            if let Ok((socket, _)) = persona_result {
                debug!("Persona connection");
                tokio::spawn(handle_client(socket, "persona"));
            }
        }
    });

    let lobby_handle = tokio::spawn(async move {
        loop {
            if !*lobby_rx.borrow() {
                debug!("Lobby listener shutting down");
                break;
            }

            let lobby_result = lobby_listener.accept().await;
            if let Err(e) = lobby_result {
                error!("Failed to accept lobby connection: {}", e);
                continue;
            }
            if let Ok((socket, _)) = lobby_result {
                debug!("Lobby connection");
                tokio::spawn(handle_client(socket, "lobby"));
            }
        }
    });

    // Main loop
    loop {
        // Check for input
        match check_for_key() {
            Keys::Quit => {
                tx.send(false).unwrap();
                break;
            }
            _ => {}
        }

        // Sleep for a bit
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    println!("Server shutting down");

    Ok(())
}
