use crate::q3_error::Q3Error;

use super::player_info::PlayerInfo;

#[derive(Debug, Clone)]
pub struct PlayerList(pub Vec<PlayerInfo>);

impl PlayerList {
    pub fn new(raw: &str) -> Result<Self, Q3Error> {
        let mut player_list: Vec<PlayerInfo> = vec![];
        for p in raw.split("\n") {
            player_list.push(PlayerInfo::new(p)?);
        }

        player_list.pop(); // Remove empty last element

        Ok(Self(player_list))
    }
}
