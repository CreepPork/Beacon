extern crate dotenv;
extern crate reqwest;

use arma_rs::{rv, rv_handler};
use dotenv::dotenv;
use std::collections;
use std::env;

#[rv(thread = true)]
fn send() {
    let payload_url = env::var("PAYLOAD_URL").expect("Failed to fetch PAYLOAD_URL.");
    let client = reqwest::blocking::Client::new();

    let mut json = collections::HashMap::new();
    json.insert("some", "data");

    let _res = client.post(&payload_url).json(&json).send();

    println!("Made request");
}

#[rv_handler]
fn init() {
    dotenv().ok();
}
