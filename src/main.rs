use crate::app::App;
use eyre::Result;

mod app;
mod cli;
mod config;
mod desktop;

fn main() -> Result<()> {
    App::new().run()
}
