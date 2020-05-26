#[macro_use]
extern crate arma_rs_macros;

use arma_rs::{rv, rv_handler};
use std::env;
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use tungstenite::Message;

mod parse;

#[rv(thread = true)]
fn start() {
    rv_callback!(
        "beacon",
        "beacon_common_fnc_log",
        "Websocket is starting on port 9001!"
    );

    let server = TcpListener::bind("127.0.0.1:9001").unwrap();

    for stream in server.incoming() {
        spawn(move || {
            let server_command_password: String = env::var("SERVER_COMMAND_PASSWORD").unwrap();
            let command_username: String = env::var("COMMAND_USERNAME").unwrap();
            let command_password: String = env::var("COMMAND_PASSWORD").unwrap();

            let websocket = accept(stream.unwrap()).unwrap();

            loop {
                let recieved_message = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages
                if recieved_message.is_binary() || recieved_message.is_text() {
                    let message = match recieved_message.to_text() {
                        Ok(m) => m,
                        Err(_) => break,
                    };

                    let blocks: Vec<&str> = message.split("|=|").collect();
                    if blocks.len() <= 3 {
                        rv_callback!(
                            "beacon",
                            "beacon_server_fnc_log",
                            "Recieved callback that had an invalid payload"
                        );
                        break;
                    }

                    let username = blocks[0];
                    let password = blocks[1];

                    if username != command_username || password != command_password {
                        rv_callback!(
                            "beacon",
                            "beacon_server_fnc_log",
                            "Recieved callback with wrong credentials"
                        );
                        break;
                    }

                    let command = blocks[2];
                    let arguments = format!("{:?}", &blocks[3..]);

                    match command {
                        "say" => {
                            rv_callback!("beacon", "beacon_commands_fnc_say", arguments);
                            websocket
                                .write_message(Message::from("You said a message."))
                                .unwrap();
                        }

                        "get-players" => {
                            rv_callback!("beacon", "beacon_commands_fnc_getPlayers", "");
                        }

                        "ban" => {
                            rv_callback!(
                                "beacon",
                                "beacon_commands_fnc_ban",
                                server_command_password,
                                arguments
                            );
                        }

                        _ => {
                            websocket
                                .write_message(Message::from("This command is not supported."))
                                .unwrap();
                        }
                    };
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
