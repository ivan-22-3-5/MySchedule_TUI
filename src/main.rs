use clap::Parser;
use cli::Cli;
use color_eyre::Result;

use crate::app::App;
use crate::persistence::schedule::JsonScheduleLoader;
use crate::persistence::settings::JsonSettingsLoader;

mod action;
mod app;
mod cli;
mod config;
mod entities;
mod errors;
mod logging;
mod persistence;
mod theme;
mod tui;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    errors::init()?;
    logging::init()?;

    let args = Cli::parse();
    let settings_loader = JsonSettingsLoader::new("./cfg");
    let schedule_loader = JsonScheduleLoader::new("./schedule");

    let mut app = App::new(
        args.tick_rate,
        args.frame_rate,
        Box::new(schedule_loader),
        Box::new(settings_loader),
    )?;

    app.run().await?;
    Ok(())
}
