# maze-server

## TODO
- Handle threads
  - https://github.com/tokio-rs/tokio/blob/master/examples/chat.rs
- Test coverage
- What to do with llvm-cov files on build
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

## Build
```bash
# local
cargo build

# docker
docker build . -t p0rt23/maze-server:latest
```

## Run
```bash
# local
RUST_LOG=trace cargo run

# docker
docker run --rm --name maze-server -p 7111:7111 p0rt23/maze-server:latest
```

## Logging
- [env_logger](https://github.com/rust-cli/env_logger/tree/main)
