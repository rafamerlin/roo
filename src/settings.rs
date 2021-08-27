use anyhow::{anyhow, Result};
use config::{Config, File, FileFormat};
use env::VarError;
use serde::Deserialize;
use std::env;
use std::error;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub commands: Vec<Commands>,
    pub walkdir: WalkDirChoice,
    pub delay: i64,
}

#[derive(Debug, Deserialize)]
pub struct Commands {
    pub key: String,
    pub command: String,
    pub command_type: CommandType,
    pub search_by: SearchBy,
    pub search_for: String,
}

#[derive(Debug, Deserialize)]
pub enum CommandType {
    File,
    Directory,
}

#[derive(Debug, Deserialize)]
pub enum WalkDirChoice {
    WalkDir,
    Fd,
}

#[derive(PartialEq, Debug, Deserialize)]
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

    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let mut s = Config::new();
        let settings_file = Self::settings_file()?;

        //Default value for walkdir
        s.set_default("walkdir", "WalkDir")?;

        s.set_default("delay", "0")?;

        // Start off by merging in the "default" configuration file
        s.merge(File::new(&settings_file, FileFormat::Yaml))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into().map_err(|e| e.into())
    }

    pub fn add_or_remove() -> Result<String, VarError> {
        let settings_file = Self::settings_file()?;
        let ret = format!("\nTo add/remove configs edit the file \"{}\" as if it was an yaml file.
\nThe `commands:` is at upper level so it always stays there.
You need to from the keys on, don't forget to escape \\ example:
\n\nwalkdir: fd
\n\ndelay: 0
commands:
  - key: \"vs\"
    command: \"C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Professional\\Common7\\IDE\\devenv.exe\"
    extension: \"sln\"
    command_arg: \"file\"
\n\nWhere:
key is what you're going to use to call roo. so in the example \"roo vs\" will run the command with key \"vs\"
command is the application the executable that will be called.
extension is the extension to look for in the list, so in our example we will look for all files with sln extension
command_arg is if we will call the command with the file as argument or with the file's directory as argument. 
", settings_file);

        Ok(ret)
    }

    pub fn list(&self) -> String {
        let mut list = String::from("Configs:\n\n");
        for c in &self.commands {
            let command = format!(
                "-------------------
Key: {}
Command: {}
Command Type: {:?}
Search for: {}
Search by:{:?}
\n",
                c.key, c.command, c.command_type, c.search_for, c.search_by
            );

            list += &command;
        }

        list
    }

    pub fn find_by_command_key(&self, command_key: &str) -> Result<&Commands> {
        self.commands
            .iter()
            .find(|x| x.key == command_key)
            .ok_or_else(|| anyhow!("Couldn't find settings with key: {}", command_key))
    }

    //todo: Add Validate config (i.e. we can't search by Directory and open by file)
}
