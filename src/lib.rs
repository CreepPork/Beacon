#[macro_use]
extern crate arma_rs_macros;

#[macro_use]
extern crate dotenv_codegen;

use arma_rs::{rv, rv_handler};
use chrono::prelude::*;
use dotenv::dotenv;
use lazy_static::lazy_static;
use tungstenite::protocol::WebSocket;
use tungstenite::server::accept;
use tungstenite::Message;

use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Mutex;
use std::thread;
use std::thread::spawn;
use std::time;

mod parse;

lazy_static! {
    static ref MESSAGE_BUFFER: Mutex<Vec<String>> = Mutex::new(vec![]);
}

#[rv(thread = true)]
fn start() {
    dotenv().ok();

    let port = dotenv!("BEACON_PORT");

    rv_callback!(
        "beacon",
        "beacon_common_fnc_log",
        format!("Websocket is starting on port {}!", port)
    );

    let server = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    for stream in server.incoming() {
        spawn(move || {
            let unwrapped_stream = stream.unwrap();

            rv_callback!(
                "beacon",
                "beacon_common_fnc_log",
                format!(
                    "Client connected from {}",
                    unwrapped_stream.local_addr().ok().unwrap()
                )
            );

            let server_command_password = dotenv!("SERVER_COMMAND_PASSWORD");
            let command_username = dotenv!("COMMAND_USERNAME");
            let command_password = dotenv!("COMMAND_PASSWORD");

            let mut websocket = accept(unwrapped_stream).unwrap();

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
                            "beacon_common_fnc_log",
                            "Recieved callback that had an invalid payload"
                        );
                        break;
                    }

                    let username = blocks[0];
                    let password = blocks[1];

                    if username != command_username || password != command_password {
                        rv_callback!(
                            "beacon",
                            "beacon_common_fnc_log",
                            "Recieved callback with wrong credentials"
                        );

                        websocket
                            .write_message(Message::from("Invalid credentials."))
                            .unwrap();

                        break;
                    }

                    let command = blocks[2];
                    let arguments = format!("{:?}", &blocks[3..]);

                    match command {
                        "messages" => {
                            // Output all messages in the buffer
                            send_buffer_messages(&mut websocket);
                        }

                        "say" => {
                            rv_callback!("beacon", "beacon_commands_fnc_say", arguments);

                            thread::sleep(time::Duration::from_secs(1));
                            send_buffer_messages(&mut websocket);
                        }

                        "get-players" => {
                            rv_callback!("beacon", "beacon_commands_fnc_getPlayers", "");

                            thread::sleep(time::Duration::from_secs(1));
                            send_buffer_messages(&mut websocket);
                        }

                        "kick" => {
                            rv_callback!(
                                "beacon",
                                "beacon_commands_fnc_kick",
                                server_command_password,
                                arguments
                            );

                            thread::sleep(time::Duration::from_secs(1));
                            send_buffer_messages(&mut websocket);
                        }

                        "ban" => {
                            rv_callback!(
                                "beacon",
                                "beacon_commands_fnc_ban",
                                server_command_password,
                                arguments
                            );

                            thread::sleep(time::Duration::from_secs(1));
                            send_buffer_messages(&mut websocket);
                        }

                        "execute" => {
                            rv_callback!("beacon", "beacon_commands_fnc_execute", arguments);

                            thread::sleep(time::Duration::from_secs(1));
                            send_buffer_messages(&mut websocket);
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

fn send_buffer_messages(websocket: &mut WebSocket<TcpStream>) {
    let mut messages = MESSAGE_BUFFER.lock().unwrap();

    if messages.len() == 0 {
        websocket
            .write_message(Message::from("No messages in the buffer."))
            .unwrap();

        return;
    }

    let message_string = messages.join("|=|");
    websocket
        .write_message(Message::from(message_string))
        .unwrap();

    messages.clear();
}

#[rv(thread = true)]
fn reply(message: String) {
    let time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    MESSAGE_BUFFER.lock().unwrap().push(time + ": " + &message);
}

#[rv(thread = true)]
fn stop() {
    //
}

#[rv_handler]
fn init() {}
