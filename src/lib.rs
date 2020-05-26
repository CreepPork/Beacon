#[macro_use]
extern crate arma_rs_macros;

use arma_rs::{rv, rv_handler};
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use tungstenite::Message;

mod parse;

#[rv(thread = true)]
fn start() {
    rv_callback!("beacon", "beacon_common_fnc_log", "pls log this");

    let server = TcpListener::bind("127.0.0.1:9001").unwrap();

    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let recieved_message = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages.
                if recieved_message.is_binary() || recieved_message.is_text() {
                    let json = match recieved_message.to_text() {
                        Ok(m) => m,
                        Err(_) => break,
                    };

                    // let sqf_code = parse::json_to_sqf(&json).unwrap();

                    websocket.write_message(Message::text(json)).unwrap();

                    rv_callback!("beacon", "beacon_server_fnc_execute", &json);
                }
            }
        });
    }
}

#[rv(thread = true)]
fn stop() {
    //
}

#[rv_handler]
fn init() {}
