use crate::error::Q3Error;
use crate::player_info::PlayerInfo;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ServerInfo {
    vars: HashMap<String, String>,
    players: Vec<PlayerInfo>,
}

impl ServerInfo {
    pub fn new(raw_info: String) -> Result<ServerInfo, Q3Error> {
        let (server_chunk, player_chunk) = raw_info.split_once('\n').unwrap_or_default();
        let vars = Self::build_server_vars(server_chunk);
        let players = Self::build_player_list(player_chunk)?;

        Ok(Self { vars, players })
    }

    pub fn vars(&self) -> &HashMap<String, String> {
        &self.vars
    }

    pub fn players(&self) -> &Vec<PlayerInfo> {
        &self.players
    }

    fn build_server_vars(server_chunk: &str) -> HashMap<String, String> {
        let mut vars: HashMap<String, String> = HashMap::new();

        let mut server_chunk = server_chunk.split('\\');
        server_chunk.next(); // Discard garbage first element

        for _i in 0..server_chunk.clone().count() {
            if let Some(key) = server_chunk.next() {
                let value = server_chunk.next().unwrap_or_default();
                vars.insert(key.to_string(), value.to_string());
            } else {
                break;
            }
        }

        vars
    }

    fn build_player_list(player_chunk: &str) -> Result<Vec<PlayerInfo>, Q3Error> {
        let mut player_list: Vec<PlayerInfo> = Vec::new();

        for p in player_chunk.split('\n') {
            // Check if the first character is a null UTF-8 byte.
            // This signals the end of the player list
            let c = p.chars().nth(0).unwrap();
            if c != '\u{0}' {
                player_list.push(PlayerInfo::new(p.to_string())?);
            }
        }

        Ok(player_list)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const VALID_PLAYER_CHUNK: &'static str =
        "11 194 \"dogue\"\n11 194 \"dogue\"\n11 194 \"dogue\"\n\u{0}";

    const VALID_SERVER_CHUNK: &'static str = /*"����statusResponse\n*/"\\players_blue\\1 2 4 \\players_red\\3 5 6 \\score_blue\\1\\score_red\\0\\score_time\\0:58\\sv_maxclients\\22\\capturelimit\\10\\fraglimit\\10\\email\\http://contact.fpsclasico.de\\url\\https://forum.fpsclasico.de\\administrator\\adminless\\dmflags\\72\\version\\Q3 1.32b.v6 linux-i386 Sep 24 2021\\g_gametype\\3\\protocol\\68\\mapname\\pro-q3dm13\\sv_hostname\\\u{7}      ! ^^--UnFreeZe^/.^-fpsclasico^/.^-de ^/(^+Oct 07^/, ^+03:07^/)\\sv_floodProtect\\1\\x_enable\\0\\bot_minplayers\\3\\www\\http://unfreeze.fpsclasico.de\\gamename\\unfreeze\\g_needpass\\0\\g_delagHitscan\\1\\g_unlaggedVersion\\2.0";

    #[test]
    fn correct_number_of_players() {
        let p = ServerInfo::build_player_list(VALID_PLAYER_CHUNK);
        assert!(p.is_ok());

        let p = p.unwrap();
        assert_eq!(p.len(), 3);
    }

    #[test]
    fn parses_server_vars() {
        let s = ServerInfo::build_server_vars(VALID_SERVER_CHUNK);
        assert_eq!(s.get("sv_maxclients").unwrap(), &"22");
        assert_eq!(s.get("fraglimit").unwrap(), &"10");
    }

    #[test]
    fn returns_server_vars() {
        let combined_chunk = format!("{}\n{}", VALID_SERVER_CHUNK, VALID_PLAYER_CHUNK);
        let s = ServerInfo::new(combined_chunk).unwrap();
        assert_eq!(s.vars().get("sv_maxclients").unwrap(), &"22");
    }

    #[test]
    fn returns_player_list() {
        let combined_chunk = format!("{}\n{}", VALID_SERVER_CHUNK, VALID_PLAYER_CHUNK);
        let s = ServerInfo::new(combined_chunk).unwrap();
        assert_eq!(s.players().len(), 3);
        assert_eq!(s.players()[0].name(), "\"dogue\"");
    }
}
