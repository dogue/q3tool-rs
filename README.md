
# Q3Tool

A zero-dependency (except `std`) pure Rust library for interacting with ioq3 (Quake 3) based game servers.

## Getting Started
### Quick Example
```rust
use q3tool::Q3Tool;

fn main() {
    let q = Q3Tool::new("someserverhost:27960");
    let server_info = q.get_status();
    
    // Print all public server c_vars
    for (k, v) in server_info.vars {
        println!("{}: {}", k, v);
    }
}
```

## Current Status

Q3Tool is still very early in development. So far it seems to work fine, but there are likely to be breaking changes in the future.
Error handling is currently non-existent. This is being worked on.

## Running the tests
Tests are somwhat lacking at the moment, but this is being worked on.

`cargo test`

## Versioning

We use [Semantic Versioning](http://semver.org/) for versioning.

## License

This project is licensed under the [MIT License](LICENSE)
