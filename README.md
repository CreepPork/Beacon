# Beacon

A Rust library which is loaded by Arma 3, as an extension, that creates a websocket server and allows full-duplex communication.

## Compile

### Windows

- `cargo build`

### Linux

- `rustup install i686-unknown-linux-gnu`
- `sudo apt install libssl-dev:i386`
- `cargo build --target i686-unknown-linux-gnu`
