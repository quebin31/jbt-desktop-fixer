use clap::Parser;
use eyre::Result;

use crate::cli::Cli;

#[derive(Debug)]
pub struct App {
    cli: Cli,
}

impl App {
    pub fn new() -> Self {
        Self { cli: Cli::parse() }
    }

    pub fn run(self) -> Result<()> {
        todo!()
    }
}
