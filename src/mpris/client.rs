use anyhow::{Context, Result};
use mpris::PlayerFinder;

pub struct MprisClient {
    finder: PlayerFinder,
}

impl MprisClient {
    pub fn new() -> Result<Self> {
        let finder = PlayerFinder::new().context("Failed to connect to D-Bus")?;
        Ok(Self { finder })
    }
    
    pub fn get_active_player(&self) -> Result<mpris::Player> {
        self.finder.find_active().context("No active media player found")
    }
}
