use crate::app::App;
use crate::logger::Logger;
use eyre::Result;

mod app;
mod cli;
mod config;
mod desktop;
mod logger;

fn main() -> Result<()> {
    Logger::init()?;
    App::new().run()
}
