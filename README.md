# Beacon

A Rust library which is loaded by Arma 3, as an extension, that creates a websocket server and allows full-duplex communication.

## Compile

### Windows

- `cargo build`
- `bash ./build.sh`

### Linux

- `sudo dpkg --add-architecture i386;`
- `sudo apt update`
- `sudo apt install libssl-dev`
- `sudo apt install libssl-dev:i386`
- `sudo apt install gcc-multilib`
- `rustup install i686-unknown-linux-gnu`
- `PKG_CONFIG_ALLOW_CROSS=1 cargo build --target i686-unknown-linux-gnu`
- `bash ./build.sh`
