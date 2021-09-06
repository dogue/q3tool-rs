use super::player_info::PlayerInfo;

#[derive(Debug, Clone)]
pub struct PlayerList(pub Vec<PlayerInfo>);

impl PlayerList {
    pub fn new(raw: &str) -> Self {
        let mut player_list: Vec<PlayerInfo> = vec![];
        for p in raw.split("\n") {
            player_list.push(PlayerInfo::new(p));
        }

        Self(player_list)
    }
}
