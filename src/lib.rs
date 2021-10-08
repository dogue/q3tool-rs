//! A Rust library for interacting with ioq3 (Quake 3) based game servers.
//!
//! Provides an interface for getting C_VARs and a player list.
//!
//! ```no_run
//! use q3tool::Q3Tool;
//!
//! # fn main() {
//! let q = Q3Tool::new("someserverhost:27960");
//! let server_info = q.get_status().unwrap();
//!    
//! // Print all public server c_vars
//! for (k, v) in server_info.vars() {
//!     println!("{}: {}", k, v);
//! }
//!
//! // Print a single server c_var
//! println!("Hostname: {}", server_info.vars().get("sv_hostname").unwrap());
//!
//! // Print all players
//! for player in server_info.players() {
//!     println!("Name: {}, Score: {}, Ping: {}", player.name(), player.score(), player.ping());
//! }
//! # }

pub mod error;
pub mod player_info;
pub mod server_info;

use crate::error::Q3Error;
use crate::server_info::ServerInfo;

use std::net;

#[derive(Debug)]
pub struct Q3Tool<'a> {
    _password: Option<&'a str>,
    host: &'a str,
}

impl<'a> Q3Tool<'a> {
    /// Creates a new instance of the Q3Tool struct but does not perform any requests
    pub fn new(host: &'a str) -> Self {
        Self {
            host,
            _password: None,
        }
    }

    /// Sends a UDP `getstatus` packet to the host and parses the response into a [ServerInfo]
    pub fn get_status(&self) -> Result<ServerInfo, Q3Error> {
        let info = Self::send_request(&self)?;
        let info = Self::parse_response(info)?;
        Ok(info)
    }

    fn create_socket(&self) -> Result<net::UdpSocket, Q3Error> {
        let socket = net::UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(&self.host)?;
        Ok(socket)
    }

    fn send_request(&self) -> Result<String, Q3Error> {
        let socket = Self::create_socket(&self)?;
        let mut buffer = [0; 2048];

        socket.send(b"\xFF\xFF\xFF\xFFgetstatus")?;
        socket.recv(&mut buffer)?;

        let info = String::from_utf8_lossy(&buffer).into_owned();

        Ok(info)
    }

    fn parse_response(raw_info: String) -> Result<ServerInfo, Q3Error> {
        if let Some((_header, info)) = raw_info.split_once('\n') {
            Ok(ServerInfo::new(info.to_string())?)
        } else {
            Err(Q3Error::InvalidResponse)
        }
    }
}
