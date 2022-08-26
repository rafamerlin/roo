mod app;
mod finder;
mod settings;

use anyhow::{anyhow, Result};
use app::Args;
use dialoguer::{theme::ColorfulTheme, Select};
use settings::Settings;
use std::path::{Path, PathBuf};
use std::process;
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;

fn main() -> Result<()> {
    let settings = Settings::new().expect("Incorrect Config file");

    match run(settings) {
        Ok(_) => {}
        Err(error) => {
            println!("{}", error);
            process::exit(-10);
        }
    }

    Ok(())
}

fn run(settings: Settings) -> Result<()> {
    let args = app::get_args();

    match &args.sub_command {
        Some(app::SubCommands::Config) => {
            println!("{}", settings.list()?);
            Ok(())
        }
        None => regular_run(args, &settings),
    }
}

fn regular_run(args: Args, settings: &Settings) -> Result<()> {
    let path = &args.path[..];
    let command_key = args
        .command
        .as_ref()
        .ok_or_else(|| anyhow!("Missing command parameter"))?;
    let command_key = &command_key[..];
    let filter_value = &args.filter;

    let selected_settings = settings.find_by_command_key(command_key)?;

    let found_paths = match settings.walkdir {
        settings::WalkDirChoice::WalkDir => finder::wd_find(selected_settings, path)?,
        settings::WalkDirChoice::Fd => finder::fd_find(selected_settings, path)?,
    };

    let found_paths = match filter_value {
        Some(filter) => found_paths
            .into_iter()
            .filter(|s| s.to_lowercase().contains(&filter.to_lowercase()))
            .collect(),
        None => found_paths,
    };

    let selected_path: &str;
    let found_count = found_paths.len();
    if found_count == 0 {
        return Err(anyhow!(
            "No matches found for '{}' in '{}'",
            command_key,
            path
        ));
    } else if found_count == 1 {
        selected_path = found_paths.first().unwrap();
    } else {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&found_paths)
            .default(0)
            .interact_opt()?;

        selected_path = match selection {
            Some(index) => &found_paths[index],
            None => return Err(anyhow!("No Entry selected")),
        };
    }

    let selected_path: PathBuf = selected_path.into();
    let command_path = get_command_path(selected_settings, &selected_path).unwrap();

    println!(
        "Executing: {} on: {:?}",
        selected_settings.command, command_path
    );

    let (tx, rx) = channel();
    if settings.delay > 0 {
        println!("Opening in {}ms", settings.delay);
    }
    let timer = timer::Timer::new();
    let _guard =
        timer.schedule_with_delay(chrono::Duration::milliseconds(settings.delay), move || {
            let _ignored = tx.send(());
        });
    rx.recv()?;

    open(selected_settings, command_path);

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn open(selected_settings: &settings::Commands, command_path: &Path) {
    Command::new(&selected_settings.command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg(command_path)
        .spawn()
        .unwrap();
}

#[cfg(target_os = "macos")]
fn open(selected_settings: &settings::Commands, command_path: &Path) {
    Command::new("open")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("-a")
        .arg(&selected_settings.command)
        .arg(command_path)
        .spawn()
        .unwrap();
}
fn get_command_path<'a>(
    selected_settings: &settings::Commands,
    path: &'a Path,
) -> Result<&'a Path> {
    let res: Result<&Path> = match (
        &selected_settings.command_type,
        &selected_settings.search_by,
    ) {
        (settings::CommandType::File, settings::SearchBy::Directory) => Err(anyhow!(
            "You can't have a command type file while searching by directory"
        )),
        (settings::CommandType::File, settings::SearchBy::File)
        | (settings::CommandType::File, settings::SearchBy::Extension) => Ok(path),
        (settings::CommandType::Directory, settings::SearchBy::File)
        | (settings::CommandType::Directory, settings::SearchBy::Extension) => path
            .parent()
            .map(|p| {
                if p.to_string_lossy() == "" {
                    Path::new(".")
                } else {
                    p
                }
            })
            .ok_or_else(|| anyhow!("Couldn't find parent folder")),
        (settings::CommandType::Directory, settings::SearchBy::Directory) => Ok(path),
    };
    res
}