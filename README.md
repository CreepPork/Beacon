# Beacon

![Build](https://github.com/CreepPork/Beacon/workflows/Build/badge.svg)

A Rust library which is loaded by Arma 3, as an extension, that creates a websocket server and allows full-duplex communication.

## Table of Contents

- [Beacon](#beacon)
  - [Table of Contents](#table-of-contents)
  - [Requirements](#requirements)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Commands](#commands)
  - [Compilation](#compilation)
    - [Windows](#windows)
    - [Linux](#linux)
  - [Security](#security)
  - [Credits](#credits)
  - [License](#license)

## Requirements

- Arma 3 (server would be preferable)
- Latest version of CBA (can be loaded as a servermod)

## Installation

- Download the version you need for your server (Windows/Linux)
- Extract the files in your `servermods` directory which is named `@beacon`
- Add it as a servermod to your launch script (e.g. `-servermod=...`)
- Copy the `.env.example` from the root of the `@beacon` folder to your server files (where the server executable is) directory as `.env`
- Configure the `.env` file
- Done!

## Usage

To use Beacon you need to connect via a web socket connection to your server and follow a simple syntax.

All commands are separated by a unique separator - `|=|`. This prevents conflicts with code.

Every single command has a minimum set of arguments required - 3:
- `username`
- `password`
- `command`
Example: `myusername|=|mypass|=|get-players`.

To find all the supported commands see the list below.

### Commands

- `messages` - Returns all messages in the message buffer. Buffer is reset after each print.
  - Returns: All the current messages in the buffer.
  - Example: `myusername|=|mypass|=|messages`

- `ban|=|<steam user ID>` - Bans a user from the server.
  - Returns: Boolean if it was successful.
  - Example: `myusername|=|mypass|=|ban|=|76561198054743530`

- `execute|=|<code>` - Executes code on the server.
  - _Note: Use single qoutes (`'`) or escape with double qoutes (`"`) your code._
  - Returns: Anything that the executed code returns.
  - Example: `myusername|=|mypass|=|execute|=|diag_log 'hello world!'`

- `get-players` - Returns all players on the server.
  - _Note: It only returns players that are in-game, the lobby doesn't return them._
  - Returns: Array in the format of - `[[<nickname>, <steam uid>, <is Curator>], [<nickname> ...]]`
  - Example: `myusername|=|mypass|=|get-players`

- `kick|=|<steam user ID>|=|<message>` - Kicks a player from the server with an optional message.
  - Returns: Boolean if it was successful.
  - Example:
    - `myusername|=|mypass|=|kick|=|76561198054743530`
    - `myusername|=|mypass|=|kick|=|76561198054743530|=|Very bad player!`

- `pm|=|<steam user ID>|=|<message>` - Sends a direct message to a single player.
  - Returns: Nothing.
  - Example: `myusername|=|mypass|=|pm|=|76561198054743530|=|Stop doing that!`

- `say|=|<message>` - Sends a global message to all players.
  - Returns: Nothing.
  - Example: `myusername|=|mypass|=|pm|=|Server is shutting down!`

## Compilation

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

## Security

If you discover any security related issues, please email [security@garkaklis.com](mailto:security@garkaklis.com) instead of using the issue tracker.

## Credits

- [Ralfs Garkaklis](https://github.com/CreepPork)
- [All Contributors](https://github.com/CreepPork/Beacon/contributors)

## License

The GNU General Public License v3.0 (GPLv3). Please see [License File](https://github.com/CreepPork/Beacon/blob/master/LICENSE) for more information.
