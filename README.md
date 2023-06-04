# redv-sifive-riscv-fe310-soc-rs

## Installation (Unix-like OS)
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add riscv32imac-unknown-none-elf
cargo install cargo-embed cargo-binutils 
```

## Build
To build an example, run the following command:
```
cargo build --example <example_name> [--release]
```
For instance, to build `blinky`:
```
cargo build --example blinky
```

## Flash
```
cargo embed --example <example_name> [--release]
```
For example, to flash `blinky`, run the following command:
```
cargo embed --example blinky
```
