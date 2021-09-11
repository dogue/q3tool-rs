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
//! // Print all players
//! for player in server_info.players() {
//!     println!("Name: {}, Score: {}, Ping: {}", player.name(), player.score(), player.ping());
//! }
//! # }

pub mod player_info;
pub mod player_list;
pub mod q3_error;
pub mod server_info;

use std::net::{self, UdpSocket};

use q3_error::Q3Error;
use server_info::ServerInfo;

#[derive(Debug)]
pub struct Q3Tool {
    password: Option<String>,
    host: String,
}

impl Q3Tool {
    pub fn new(host: &str) -> Self {
        Self {
            host: host.to_owned(),
            password: None,
        }
    }

    pub fn new_with_rcon(host: &str, password: &str) -> Self {
        Self {
            host: host.to_owned(),
            password: Some(password.to_owned()),
        }
    }

    pub fn get_status(&self) -> Result<ServerInfo, Q3Error> {
        let socket = self.get_socket()?;

        let mut buffer = [0; 2048];

        socket.send(b"\xFF\xFF\xFF\xFFgetstatus")?;
        socket.recv(&mut buffer)?;

        let raw_info = String::from_utf8_lossy(&buffer);
        let (_header, raw_info) = raw_info.split_once("\n").unwrap();

        let server_info = ServerInfo::new(raw_info)?;

        Ok(server_info)
    }

    pub fn send_rcon(&self, command: &str) -> Result<String, Q3Error> {
        let password = match &self.password {
            Some(password) => password,
            None => return Err(Q3Error::UserError),
        };

        let socket = self.get_socket()?;

        let mut buffer = [0; 2048];
        let prefix = b"\xFF\xFF\xFF\xFFrcon ";
        let suffix = format!("{} {}", &password, &command);

        socket.send(&[prefix, suffix.as_bytes()].concat()[..])?;
        socket.recv(&mut buffer)?;

        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    fn get_socket(&self) -> Result<UdpSocket, Q3Error> {
        let socket = net::UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(&self.host)?;
        Ok(socket)
    }
}

#[cfg(test)]
mod tests {}
