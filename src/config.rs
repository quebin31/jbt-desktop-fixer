use std::collections::HashMap;
use std::path::Path;

use eyre::Context;
use eyre::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub apps: HashMap<String, IconOrVariants>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IconOrVariants {
    Icon(String),
    Variants(HashMap<String, String>),
}

impl Config {
    const DEFAULT_CONFIG_STR: &str = include_str!("../share/config.toml");

    pub fn default() -> Result<Self> {
        let config_path = dirs::config_dir()
            .map(|p| p.join("jbt-desktop-fixer/config.toml"))
            .filter(|p| p.exists());

        if let Some(path) = config_path {
            Self::from_path(path)
        } else {
            toml::from_str(Self::DEFAULT_CONFIG_STR).wrap_err("Failed to parse default config")
        }
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let contents = std::fs::read_to_string(path).wrap_err("Failed to open config file")?;
        toml::from_str(&contents).wrap_err("Failed to parse config")
    }

    pub fn icon_name_for<'a>(&'a self, name: &str) -> Option<&'a str> {
        let name = name.to_lowercase();
        for (app_key, icon_or_variants) in self.apps.iter() {
            match icon_or_variants {
                IconOrVariants::Icon(icon) => {
                    if name.contains(app_key) {
                        return Some(icon);
                    }
                }

                IconOrVariants::Variants(variants) => {
                    if !name.contains(app_key) {
                        continue;
                    }

                    let mut default_icon = None;
                    for (variant_key, variant_icon) in variants.iter() {
                        if variant_key == "default" {
                            default_icon = Some(variant_icon);
                        } else if name.contains(variant_key) {
                            return Some(variant_icon);
                        }
                    }

                    if let Some(default_icon) = default_icon {
                        return Some(default_icon);
                    }
                }
            }
        }

        None
    }
}
