use crate::q3_error::Q3Error;

use super::player_list::PlayerList;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ServerInfo {
    /// Keys of the `HashMap` are the same as they appear in raw output from the server. See the (truncated) example below.
    ///
    /// ```plain
    /// fraglimit: 0
    /// g_waverespawns: 0
    /// g_redwave: 15
    /// g_stratTime: 5
    /// timelimit: 20
    /// capturelimit: 8
    /// g_roundtime: 2
    /// sv_hostname: ^7|^1RFA^7| ^2RisenFromAshes.us
    /// sv_maxPing: 0
    /// ```
    pub vars: HashMap<String, String>,
    pub players: PlayerList,
}

impl ServerInfo {
    pub fn new(raw: &str) -> Result<Self, Q3Error> {
        let (server_raw, player_raw) = raw.split_once("\n").unwrap_or_default();
        let players = PlayerList::new(player_raw)?;

        let mut vars: HashMap<String, String> = HashMap::new();
        let mut server_raw = server_raw.split("\\");

        // Remove empty first element
        server_raw.next();

        for _i in 0..server_raw.clone().count() {
            if let Some(key) = server_raw.next() {
                let value = server_raw.next().unwrap_or_else(|| &"");
                vars.insert(String::from(key), String::from(value));
            } else {
                break;
            }
        }

        Ok(Self { vars, players })
    }
}
