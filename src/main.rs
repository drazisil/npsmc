use std::net::TcpListener;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
#[macro_use]
extern crate log as log_crate;
extern crate simplelog;

use crate::{log::init_logging, net::handle_client};

// Struct to hold server information
struct Server {
    name: String,

    listener: TcpListener,
}

mod log;
mod net;
mod packet;

fn main() -> std::io::Result<()> {
    let login_port = 8226;
    let persona_port = 8228;
    let lobby_port = 7003;

    init_logging();

    println!("Starting server");
    println!("Press ? for help");
    println!("Press x to quit");

    let mut listeners = Vec::new();

    let login_server = Server {
        name: "login".to_string(),
        listener: TcpListener::bind(("0.0.0.0", login_port)).unwrap(),
    };
    debug!("Listening on port {}", login_port);

    let persona_server = Server {
        name: "persona".to_string(),
        listener: TcpListener::bind(("0.0.0.0", persona_port)).unwrap(),
    };
    debug!("Listening on port {}", persona_port);

    let lobby_server = Server {
        name: "lobby".to_string(),
        listener: TcpListener::bind(("0.0.0.0", lobby_port)).unwrap(),
    };
    debug!("Listening on port {}", lobby_port);

    listeners.push(login_server);
    listeners.push(persona_server);
    listeners.push(lobby_server);

    for listener in &listeners {
        listener.listener.set_nonblocking(true).unwrap();
    }

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
        for listener in &listeners {
            match listener.listener.accept() {
                Ok((stream, _)) => {
                    handle_client(stream, &listener.name);
                }
                Err(_) => {
                    // Free up the thread
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
    }

    println!("Server shutting down");

    Ok(())
}


