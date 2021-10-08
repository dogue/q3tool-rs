use crate::error::Q3Error;

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    name: String,
    score: i32,
    ping: i32,
}

impl PlayerInfo {
    pub fn new(player: String) -> Result<Self, Q3Error> {
        let mut player = player.splitn(3, ' ');

        let score = player.next().unwrap_or_default().parse()?;
        let ping = player.next().unwrap_or_default().parse()?;
        let name = player.next().unwrap_or_default().to_string();

        Ok(Self { name, score, ping })
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn ping(&self) -> i32 {
        self.ping
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const VALID_PLAYER: &'static str = "11 194 \"dogue\"";
    const INVALID_SCORE: &'static str = "a 194 \"dogue\"";
    const INVALID_PING: &'static str = "11 a \"dogue\"";

    #[test]
    fn parses_score() {
        let p = PlayerInfo::new(VALID_PLAYER.to_string());
        assert!(p.is_ok());
        assert_eq!(11, p.unwrap().score());
    }

    #[test]
    fn parses_ping() {
        let p = PlayerInfo::new(VALID_PLAYER.to_string());
        assert!(p.is_ok());
        assert_eq!(194, p.unwrap().ping());
    }

    #[test]
    fn parses_name() {
        let p = PlayerInfo::new(VALID_PLAYER.to_string());
        assert!(p.is_ok());
        assert_eq!("\"dogue\"", p.unwrap().name());
    }

    #[test]
    fn parseint_error_on_invalid_score() {
        let p = PlayerInfo::new(INVALID_SCORE.to_string());
        assert!(p.is_err());
        assert!(matches!(p.err(), Some(Q3Error::ParseError(_))));
    }

    #[test]
    fn parseint_error_on_invalid_ping() {
        let p = PlayerInfo::new(INVALID_PING.to_string());
        assert!(p.is_err());
        assert!(matches!(p.err(), Some(Q3Error::ParseError(_))))
    }
}
