# Beacon

A Rust library which is loaded by Arma 3, as an extension, that creates a websocket server and allows full-duplex communication.

## Requirements

- Arma 3 (server would be preferable)

## Installation

- Download the version you need for your server (Windows/Linux)
- Extract the files in your `servermods` directory which is named `@beacon`
- Add it as a servermod to your launch script (e.g. `-servermod=...`)
- Copy the `.env.example` from the root of the `@beacon` folder to your server files (where the server executable is) directory as `.env`
- Configure the `.env` file
- Done!

## Compile

For development purposes:

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
