pub mod player_info;
pub mod player_list;
pub mod server_info;

use std::net;

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

    pub fn get_status(&self) -> ServerInfo {
        let socket = net::UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.connect(&self.host).unwrap();

        let mut buffer = [0; 2048];

        socket.send(b"\xFF\xFF\xFF\xFFgetstatus").unwrap();
        socket.recv(&mut buffer).unwrap();

        let raw_info = String::from_utf8_lossy(&buffer);
        let (_header, raw_info) = raw_info.split_once("\n").unwrap();

        ServerInfo::new(raw_info)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
