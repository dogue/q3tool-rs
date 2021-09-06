#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub name: String,
    pub score: i32,
    pub ping: i32,
}

impl PlayerInfo {
    pub fn new(player: &str) -> Self {
        let mut player = player.splitn(3, " ");
        let score = player
            .next()
            .unwrap_or_else(|| &"")
            .parse()
            .unwrap_or_default();
        let ping = player
            .next()
            .unwrap_or_else(|| &"")
            .parse()
            .unwrap_or_default();
        let name = String::from(player.next().unwrap_or_else(|| &""));

        Self { name, score, ping }
    }
}
