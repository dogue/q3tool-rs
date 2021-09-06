use super::player_list::PlayerList;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub vars: HashMap<String, String>,
    pub players: PlayerList,
}

impl ServerInfo {
    pub fn new(raw: &str) -> Self {
        let (server_raw, player_raw) = raw.split_once("\n").unwrap();
        let players = PlayerList::new(player_raw);

        let mut vars: HashMap<String, String> = HashMap::new();
        let mut server_raw = server_raw.split("\\");
        server_raw.next(); // Remove empty first element

        for _i in 0..server_raw.clone().count() {
            if let Some(key) = server_raw.next() {
                let value = server_raw.next().unwrap_or_else(|| &"");
                vars.insert(String::from(key), String::from(value));
            } else {
                break;
            }
        }

        Self { vars, players }
    }
}
