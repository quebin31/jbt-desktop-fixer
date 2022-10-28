use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use eyre::Context;
use eyre::ContextCompat;
use eyre::Result;

#[derive(Debug, Clone)]
pub struct DesktopEntry {
    pub entry_path: PathBuf,
    pub name: String,
    pub icon: Option<String>,
}

impl DesktopEntry {
    const NAME_KEY: &str = "Name";
    const ICON_KEY: &str = "Icon";

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path.as_ref()).wrap_err("Failed to open desktop entry")?;
        let reader = BufReader::new(file);
        let mut values = HashMap::new();

        for line in reader.lines().skip(1) {
            let line = line
                .map(|s| s.trim().to_owned())
                .wrap_err("Failed to read desktop entry")?;

            let split: Vec<_> = line.split('=').collect();
            let key = split.first().wrap_err("No key found")?;
            match *key {
                Self::NAME_KEY | Self::ICON_KEY => {
                    let value = split.get(1).wrap_err("No value found")?;
                    values.insert(key.to_string(), value.to_string());
                }

                _ => {
                    // no-op
                }
            }
        }

        let entry_path = path.as_ref().to_path_buf();
        let name = values
            .get(Self::NAME_KEY)
            .wrap_err("No key 'Name' was found in this desktop entry")?
            .to_string();
        let icon = values.get(Self::ICON_KEY).map(|s| s.to_string());

        Ok(Self {
            entry_path,
            name,
            icon,
        })
    }

    pub fn modify_icon(&self, icon_name: &str) -> Result<()> {
        if let Some(current_icon_name) = &self.icon {
            if icon_name == current_icon_name {
                return Ok(());
            }
        }

        log::info!("Updating icon for {}", self.name);

        let contents =
            std::fs::read_to_string(&self.entry_path).wrap_err("Failed to read desktop entry")?;
        let file = File::create(&self.entry_path).wrap_err("Failed to open file in write mode")?;
        let mut writer = BufWriter::new(file);

        for line in contents.split('\n') {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with(Self::ICON_KEY) {
                writeln!(writer, "{}={}", Self::ICON_KEY, icon_name)?;
            } else {
                writeln!(writer, "{}", line)?;
            }
        }

        writeln!(writer)?;
        Ok(writer.flush()?)
    }
}
