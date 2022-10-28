use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use eyre::Context;
use eyre::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub apps: Vec<JbApp>,
}

#[derive(Debug, Deserialize)]
pub struct JbApp {
    pub descriptor: String,
    #[serde(flatten)]
    pub single_or_multiple: SingleOrMultiple,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SingleOrMultiple {
    Single { icon_name: String },
    Multiple { variants: Vec<Variant> },
}

#[derive(Debug, Deserialize)]
pub struct Variant {
    pub descriptor: Option<String>,
    pub icon_name: String,
}

impl Config {
    const DEFAULT_CONFIG_STR: &str = include_str!("../share/config.yaml");

    pub fn default() -> Result<Self> {
        serde_yaml::from_str(Self::DEFAULT_CONFIG_STR).wrap_err("Failed to parse default config")
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path).wrap_err("Failed to open config file")?;
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).wrap_err("Failed to parse config")
    }

    pub fn icon_name_for<'a>(&'a self, name: &str) -> Option<&'a str> {
        let name = name.to_lowercase();
        for app in self.apps.iter() {
            match &app.single_or_multiple {
                SingleOrMultiple::Single { icon_name } => {
                    if name.contains(&app.descriptor) {
                        return Some(icon_name);
                    }
                }

                SingleOrMultiple::Multiple { variants } => {
                    if !name.contains(&app.descriptor) {
                        continue;
                    }

                    for variant in variants.iter() {
                        if let Some(descriptor) = &variant.descriptor {
                            if name.contains(descriptor) {
                                return Some(&variant.icon_name);
                            }
                        } else {
                            return Some(&variant.icon_name);
                        };
                    }
                }
            }
        }

        None
    }
}
