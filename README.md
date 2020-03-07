# Pacman Mirrors Rust
This is a port of the Manjaro pacman-mirrors tool written in python.

## Usage
Right now, the feature set is really incomplete but I aim to achieve
parity with the python tool with minor differences, such as a default
timeout in fasttrack mode.

## Build
To build the tool you can just use the following:
```
cargo build --release
```

In order to reduce the binary size, it can also be built with Xargo
and then stripped.
```
xargo build --target x86_64-unknown-linux-gnu --release
strip target/x86_64-unknown-linux-gnu/release/pacman-mirrors
```

## Why?
I decided to port the tool to Rust in order to challenge myself to
understand another tool and to learn how to use async/await Rust.
