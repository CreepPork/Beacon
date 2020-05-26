#!/bin/bash

build_windows() {
    if [ "$1" == "--release" ]; then
        cargo build --release
        cp ./target/release/beacon.dll ./beacon_x64.dll
    else
        cargo build
        cp ./target/debug/beacon.dll ./beacon_x64.dll
    fi
};

build_linux() {
    if [ "$1" == "--release" ]; then
        PKG_CONFIG_ALLOW_CROSS=1 cargo build --target i686-unknown-linux-gnu --release
        cp ./target/i686-unknown-linux-gnu/release/libbeacon.so ./beacon.so
    else
        PKG_CONFIG_ALLOW_CROSS=1 cargo build --target i686-unknown-linux-gnu
        cp ./target/i686-unknown-linux-gnu/debug/libbeacon.so ./beacon.so
    fi
};

if [[ "$OSTYPE" == "msys" ]]; then
    build_windows "$1"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    build_linux "$1"
else
    echo "OS Type is not supported by this utility."
fi
