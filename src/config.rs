use glib::translate::{FromGlib, ToGlib};
use serde::{Deserialize, Serialize};

/// Button event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Lock,
    Logout,
    Reboot,
    Shutdown,
    Suspend,
    Custom(Vec<String>),
}

/// Config of gtk button
#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    pub icon: String,
    pub event: Event,
    pub key: Key,
}

/// Keyboard key wrapper
#[derive(Debug)]
pub struct Key(gdk::keys::Key);

impl Serialize for Key {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if let Some(k) = gdk::keyval_name(self.0.to_glib()) {
            serializer.serialize_str(&k.to_string())
        } else {
            panic!("test")
        }
    }
}
impl<'a> Deserialize<'a> for Key {
    fn deserialize<D: serde::Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let val = gdk::keyval_from_name(&s);
        let key = gdk::keys::Key::from_glib(val);

        if key == gdk::keys::constants::VoidSymbol {
            Err(serde::de::Error::unknown_variant(
                &s,
                &["One of gdk key names"],
            ))
        } else {
            Ok(Key(key))
        }
    }
}

impl std::ops::Deref for Key {
    type Target = gdk::keys::Key;
    fn deref(&self) -> &gdk::keys::Key {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_time_format")]
    pub time_format: String,
    #[serde(default = "default_date_format")]
    pub date_format: String,
    #[serde(default)]
    pub buttons: Vec<Button>,
}

fn default_time_format() -> String {
    "%H:%M".to_string()
}
fn default_date_format() -> String {
    "%A, %d %B %Y".to_string()
}

impl Config {
    pub fn deserialize(input: &str) -> Self {
        let ron: Self = match ron::from_str(&input) {
            Ok(ron) => ron,
            Err(err) => {
                let msg = format!("{:#?}", err);
                crate::quick_dialog(&msg);
                panic!("{}", msg);
            }
        };

        ron
    }
}

pub fn get_config() -> (Config, Vec<u8>) {
    let config_dir = config_dir();

    let default_config = include_str!("./config.ron");
    let default_style = include_bytes!("./style.css");

    let (ron_str, style_str) = if let Some(config_dir) = config_dir {
        let bar_config_dir = config_dir.join("YetAnotherPowerMenu");
        let _ = std::fs::create_dir_all(&bar_config_dir);
        let ron_str: String =
            if let Ok(file) = std::fs::read_to_string(&bar_config_dir.join("config.ron")) {
                file
            } else {
                default_config.into()
            };
        let style_str = if let Ok(file) = std::fs::read(&bar_config_dir.join("style.css")) {
            file
        } else {
            default_style.to_vec()
        };
        (ron_str, style_str)
    } else {
        (default_config.into(), default_style.to_vec())
    };

    let decoded = Config::deserialize(&ron_str);

    (decoded, style_str)
}

use std::path::PathBuf;
pub fn config_dir() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").and_then(|h| if h.is_empty() { None } else { Some(h) });
    if let Some(home) = home {
        Some(PathBuf::from(home).join(".config"))
    } else {
        None
    }
}
