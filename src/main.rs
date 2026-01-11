mod app;
mod audio;
mod config;
mod mpris;
mod ui;
mod visualizers;

use anyhow::Result;
use env_logger::Env;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("🎸 Starting ArchyNotch...");
    let config = config::Config::load_or_default()?;
    app::ArchyNotch::run(config)?;
    Ok(())
}
