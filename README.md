# Beacon

A Rust library which is loaded by Arma 3, as an extension, that creates a websocket server and allows full-duplex communication.

## Compile

### Windows

- `cp .env.example .env`
- Configure the `.env` file
- `cargo build`
- `bash ./build.sh`

### Linux

- `cp .env.example .env`
- Configure the `.env` file
- `sudo dpkg --add-architecture i386;`
- `sudo apt update`
- `sudo apt install libssl-dev`
- `sudo apt install libssl-dev:i386`
- `sudo apt install gcc-multilib`
- `rustup install i686-unknown-linux-gnu`
- `bash ./build.sh`
