
# Q3Tool

A Rust library for interacting with ioq3 (Quake 3) based game servers.

[![Rust](https://github.com/dogue/q3tool-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/dogue/q3tool-rs/actions/workflows/rust.yml)
[![GitHub issues](https://img.shields.io/github/issues/dogue/q3tool-rs.svg)](https://GitHub.com/dogue/q3tool-rs/issues/)
[![MIT license](https://img.shields.io/badge/License-MIT-blue.svg)](https://lbesson.mit-license.org/)

## Getting Started
### Quick Example
```rust
use q3tool::Q3Tool;

fn main() {
    let q = Q3Tool::new("someserverhost:27960");
    let server_info = q.get_status().unwrap();
    
    // Print all public server c_vars
    for (k, v) in server_info.vars() {
        println!("{}: {}", k, v);
    }

    // Print a single server c_var
    println!("Hostname: {}", server_info.vars().get("sv_hostname").unwrap());

    // Print all players
    for player in server_info.players() {
        println!("Name: {}, Score: {}, Ping: {}", player.name(), player.score(), player.ping());
    }
}
```

## Current Status

Q3Tool is still in development.

There *shouldn't* be any breaking changes in the public facing API at this point.

## Running the tests
`cargo test`

## License

This project is licensed under the [MIT License](LICENSE)
