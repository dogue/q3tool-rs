use crate::q3_error::Q3Error;

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    name: String,
    score: i32,
    ping: i32,
}

impl PlayerInfo {
    pub fn new(player: &str) -> Result<Self, Q3Error> {
        let mut player = player.splitn(3, ' ');

        let score = player.next().unwrap_or("0").parse().unwrap_or_default();
        let ping = player.next().unwrap_or("0").parse().unwrap_or_default();
        let name = String::from(player.next().unwrap_or("0"));

        Ok(Self { name, score, ping })
    }

    /// Names are passed as-is, meaning color escape codes and double quotes are still present.
    /// A method for stripping/interpreting these *may* be implemented in the future.
    ///
    /// [See here](https://www.computerhope.com/issues/ch000658.htm) for more information on Q3 color codes.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Player frag count. Represented in-engine as "score".
    pub fn score(&self) -> &i32 {
        &self.score
    }

    /// Player latency
    pub fn ping(&self) -> &i32 {
        &self.ping
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
