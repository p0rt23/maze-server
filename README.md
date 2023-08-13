# maze-server

## TODO
- Create config file
    - Compare [this project](https://github.com/astriaorg/astria-conductor/blob/main/astria-conductor/src/bin/conductor.rs)
- Create logging
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
```bash
vi App.toml
```

## Test
```bash
# Run unit tests
cargo llvm-cov
```

## Run
```bash
cargo run
```
