use crate::q3_error::Q3Error;

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    /// Names are passed as-is, meaning color escape codes are still present.
    /// A method for stripping these *may* be implemented in the future.
    pub name: String,
    pub score: i32,
    pub ping: i32,
}

impl PlayerInfo {
    pub fn new(player: &str) -> Result<Self, Q3Error> {
        let mut player = player.splitn(3, " ");
        let score = player.next().unwrap_or_else(|| &"").parse()?;
        let ping = player.next().unwrap_or_else(|| &"").parse()?;
        let name = String::from(player.next().unwrap_or_else(|| &""));

        Ok(Self { name, score, ping })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_STR: &'static str = "11 194 \"killer\"";

    #[test]
    fn extract_score() {
        let p = PlayerInfo::new(TEST_STR).unwrap();
        assert_eq!(11, p.score);
    }

    #[test]
    fn extract_ping() {
        let p = PlayerInfo::new(TEST_STR).unwrap();
        assert_eq!(194, p.ping);
    }

    #[test]
    fn extract_name() {
        let p = PlayerInfo::new(TEST_STR).unwrap();
        assert_eq!("\"killer\"", p.name);
    }
}
