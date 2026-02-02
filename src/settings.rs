use anyhow::{anyhow, Result};
use config::{Config, File, FileFormat};
use env::VarError;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub commands: Vec<Commands>,
    pub delay: i64,
}

#[derive(Debug, Deserialize)]
pub struct Commands {
    pub key: String,
    pub command: String,
    pub command_type: CommandType,
    pub search_by: SearchBy,
    #[serde(deserialize_with = "deserialize_search_for")]
    pub search_for: Vec<String>,
}

fn deserialize_search_for<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum SearchForValue {
        String(String),
        Vec(Vec<String>),
    }

    match SearchForValue::deserialize(deserializer)? {
        SearchForValue::String(s) => Ok(vec![s]),
        SearchForValue::Vec(v) => Ok(v),
    }
}

#[derive(Debug, Deserialize)]
pub enum CommandType {
    File,
    Directory,
}

#[derive(Eq, PartialEq, Debug, Deserialize)]
pub enum SearchBy {
    File,
    Directory,
    Extension,
}

impl Settings {
    fn settings_file() -> Result<String, VarError> {
        let arg = dirs_next::home_dir()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        Ok(arg + "/.roo")
    }

    pub fn new() -> Result<Self> {
        let settings_file = Self::settings_file()?;

        let settings = Config::builder()
            .add_source(File::new(&settings_file, FileFormat::Yaml))
            .set_default("delay", "0")?
            .build()?
            .try_deserialize()?;

        Ok(settings)
    }

    pub fn list(&self) -> Result<String> {
        let settings_file = Self::settings_file()?;

        let mut list = format!("Location: {}\n\nConfigs:\n\n", settings_file);
        for c in &self.commands {
            let command = format!(
                "-------------------
Key: {}
Command: {}
Command Type: {:?}
Search for: {:?}
Search by: {:?}
\n",
                c.key, c.command, c.command_type, c.search_for, c.search_by
            );

            list += &command;
        }

        Ok(list)
    }

    pub fn find_by_command_key(&self, command_key: &str) -> Result<&Commands> {
        self.commands
            .iter()
            .find(|x| x.key == command_key)
            .ok_or_else(|| anyhow!("Couldn't find settings with key: {}", command_key))
    }
}
