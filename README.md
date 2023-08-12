# maze-server

## TODO
- Unit testing
- Create config file
    - How do rust projects organize files?
    - Where do I put a config file, does that get copied somewhere for release?
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
