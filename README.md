# maze-server

## TODO
- Server loop and console commands (quit)
    - Run in a docker container
    - Setup jenkins build
- Establish rectangle map size
    - Game data struct
- Listen on port
- Register client connection
    - Set client position
    - Send game data
    - Handle client leave
    - Command: list clients, kick [client-id]

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
