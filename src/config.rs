use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub window: WindowConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
}

impl Config {
    pub fn load_or_default() -> Result<Self> {
        Ok(Self {
            window: WindowConfig {
                width: 320,
                height: 90,
                x: 800,
                y: 0,
            },
        })
    }
}
