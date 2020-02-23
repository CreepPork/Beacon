extern crate dotenv;

use arma_rs::{rv, rv_handler};
use dotenv::dotenv;
use std::env;

#[rv]
fn hello() -> &'static str {
    "Hello from Rust!"
}

#[rv]
fn is_arma3(version: u8) -> bool {
    version == 3
}

#[rv_handler]
fn init() {
    dotenv().ok();
}
