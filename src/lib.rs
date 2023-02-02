//! A Rust library for interacting with ioq3 (Quake 3) based game servers.
//!
//! Provides an interface for getting C_VARs and a player list.
//!
//! ```no_run
//! use q3tool::Q3Tool;
//!
//! # fn main() {
//! let q = Q3Tool::new("someserverhost:27960", Some("supersecretpassword"));
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
//!
//! // Send an rcon command
//! let response = q.rcon("map ut4_casa").unwrap();
//! # }

pub mod error;
pub mod player_info;
pub mod server_info;

use crate::error::Q3Error;
use crate::server_info::ServerInfo;

use format_bytes::format_bytes;
use std::net;

#[derive(Debug)]
pub struct Q3Tool {
    password: Option<String>,
    host: String,
}

impl Q3Tool {
    /// Creates a new instance of the Q3Tool struct but does not perform any requests
    pub fn new(host: &str, password: Option<String>) -> Self {
        Self {
            host: host.to_owned(),
            password,
        }
    }

    /// Sends a UDP `getstatus` packet to the host and parses the response into a [ServerInfo]
    pub fn get_status(&self) -> Result<ServerInfo, Q3Error> {
        let info = self.send_request()?;
        let info = Self::parse_response(info)?;
        Ok(info)
    }

    /// Sends an RCON command to the host.
    /// Returns the server response as a String.
    pub fn rcon(&self, command: &str) -> Result<String, Q3Error> {
        let socket = self.create_socket()?;
        let mut buffer = [0; 2048];

        let request = format_bytes!(
            b"\xFF\xFF\xFF\xFFrcon {} {}",
            self.password.as_ref().unwrap().as_bytes(),
            command.as_bytes()
        );
        socket.send(&request)?;
        socket.recv(&mut buffer)?;

        let response = String::from_utf8_lossy(&buffer).into_owned();

        Ok(response)
    }

    fn create_socket(&self) -> Result<net::UdpSocket, Q3Error> {
        let socket = net::UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(&self.host)?;
        Ok(socket)
    }

    fn send_request(&self) -> Result<String, Q3Error> {
        let socket = self.create_socket()?;
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
