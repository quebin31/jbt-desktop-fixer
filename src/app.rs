use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use eyre::Context;
use eyre::ContextCompat;
use eyre::Result;
use notify::Event;
use notify::EventKind;
use notify::Watcher;

use crate::cli::Cli;
use crate::cli::Command;
use crate::config::Config;
use crate::desktop::DesktopEntry;

#[derive(Debug)]
pub struct App {
    cli: Cli,
}

impl App {
    const SERVICE_FILE_CONTENTS: &str = include_str!("../share/jbt-desktop-fixer.service");

    pub fn new() -> Self {
        Self { cli: Cli::parse() }
    }

    pub fn run(self) -> Result<()> {
        match &self.cli.command {
            Command::CopyService => self.copy_service(),
            Command::Watch { config } => self.watch(config),
        }
    }

    fn watch(&self, config_path: &Option<PathBuf>) -> Result<()> {
        let config = if let Some(path) = config_path {
            Config::from_path(path)?
        } else {
            Config::default()?
        };

        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher =
            notify::recommended_watcher(tx).wrap_err("Couldn't start file watcher")?;

        let applications_dir = dirs::data_local_dir()
            .wrap_err("Couldn't get local data directory")?
            .join("applications");

        let paths: Vec<_> = applications_dir
            .read_dir()
            .wrap_err("Couldn't read applications dir")?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect();

        Self::update_icons(&paths, &config);

        watcher
            .watch(&applications_dir, notify::RecursiveMode::NonRecursive)
            .wrap_err("Couldn't watch the local applications dir")?;

        log::info!("Watching desktop entries");
        for res in rx {
            match res {
                Ok(event) => Self::handle_event(event, &config),
                Err(e) => log::error!("Watch error: {e:?}"),
            }
        }

        Ok(())
    }

    fn handle_event(event: Event, config: &Config) {
        if event.need_rescan() {
            return;
        }

        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                Self::update_icons(&event.paths, config);
            }
            _ => {
                // no-op
            }
        }
    }

    fn update_icons(paths: &[impl AsRef<Path>], config: &Config) {
        let entries: Vec<_> = paths
            .iter()
            .map(|p| p.as_ref())
            .filter(|p| {
                if let Some(name) = p.file_name() {
                    p.is_file() && name.to_string_lossy().contains("jetbrains")
                } else {
                    false
                }
            })
            .filter_map(|p| DesktopEntry::from_path(p).ok())
            .collect();

        for entry in entries {
            if let Some(icon_name) = config.icon_name_for(&entry.name) {
                if let Err(e) = entry.modify_icon(icon_name) {
                    log::error!("Failed to modify desktop entry: {e:?}");
                }
            }
        }
    }

    fn copy_service(&self) -> Result<()> {
        log::info!("Copying service file");
        let destination = dirs::config_dir()
            .wrap_err("Couldn't get default config dir")?
            .join("systemd/user/jbt-desktop-fixer.service");

        let mut file = File::create(&destination).wrap_err("Couldn't create service file")?;
        write!(file, "{}", Self::SERVICE_FILE_CONTENTS)
            .wrap_err("Couldn't copy service contents")?;

        log::info!("Service file copied");
        Ok(())
    }
}
