# passman-cli 

A secure CLI password manager written in Rust.

## Features
- Master password protected vault
- Argon2 key derivation
- AES-256-GCM authenticated encryption
- Encrypted local storage
- Secure password input (no echo)
- Commands: add, get, list, delete

## Usage
```bash
cargo run -- add <service> <username>
cargo run -- get <service>
cargo run -- list
cargo run -- delete <service>
