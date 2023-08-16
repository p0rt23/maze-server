# maze-server

## TODO
- Handle threads
  - https://github.com/tokio-rs/tokio/blob/master/examples/chat.rs
- Get rid of unwraps
- Test coverage
- Register client connection
    - Set client position
    - Send game data
    - Handle client leave
    - Command: list clients, kick [client-id]
- Setup jenkins build
- Establish rectangle map size
    - Game data struct

## Install
```bash
# Install prerequisites
./setup.sh
```

## Configure

Application configuration:
```bash
vi App.toml
```
Log levels can be set via `RUST_LOG` environment variable.

## Test
```bash
# Run unit tests
cargo llvm-cov
```

## Run
```bash
RUST_LOG=trace cargo run
```

## Logging
- [env_logger](https://github.com/rust-cli/env_logger/tree/main)
