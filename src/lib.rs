use arma_rs::{rv, rv_handler};
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;

#[rv(thread=true)]
fn start_server() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();

    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}

#[rv_handler]
fn init() {}