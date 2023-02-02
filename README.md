
# Q3Tool

A Rust library for interacting with ioq3 (Quake 3) based game servers.

[![GitHub issues](https://img.shields.io/github/issues/dogue/q3tool-rs.svg)](https://GitHub.com/dogue/q3tool-rs/issues/)
[![MIT license](https://img.shields.io/badge/License-MIT-blue.svg)](https://lbesson.mit-license.org/)

## Getting Started
### Quick Example
```rust
use q3tool::Q3Tool;

fn main() {
    let q = Q3Tool::new("someserverhost:27960", Some("supersecretpassword".to_owned()));
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

    let response = q.rcon("map ut4_casa").unwrap();
}
```

## Current Status

Q3Tool is essentially feature-complete. RCON is finally implemented and it is able to retrieve data from any compatible ioq3 server that I can test. If you encounter any bugs, please don't hesitate to open an issue.

There *shouldn't* be any breaking changes in the public facing API at this point.

## Running the tests
`cargo test`

## License

This project is licensed under the [MIT License](LICENSE)
