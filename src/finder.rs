use std::process::Command;

use crate::settings::{Commands, SearchBy};
use anyhow::Result;
use walkdir::{DirEntry, WalkDir};

pub fn wd_find(command: &Commands, path: &str) -> Result<Vec<String>> {
    let matching_files = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| filter_dir_entry(e, command))
        //I need to find a way to handle this as a result instead of unwraping it here.
        .map(|e| e.path().to_str().unwrap().to_owned())
        .collect();

    Ok(matching_files)
}

pub fn fd_find(command: &Commands, path: &str) -> Result<Vec<String>> {
    let output = Command::new("fd")
        .arg("-i")
        .args(match command.search_by {
            SearchBy::File => ["-t", "f", &command.search_for],
            SearchBy::Directory => ["-t", "d", &command.search_for],
            SearchBy::Extension => ["-e", &command.search_for, "."],
        })
        .arg(path)
        .output()
        .expect("Failed to execute FD. Check if it's installed");

    let resp = String::from_utf8(output.stdout)?;
    let mut lines: Vec<String> = resp.lines().into_iter().map(|s| s.to_owned()).collect();
    lines.sort_by_key(|a| a.to_lowercase());

    Ok(lines)
}

fn is_hidden(entry: &DirEntry) -> bool {
    let path = entry.path();
    let file_name = match path.file_name() {
        Some(file_name) => file_name,
        None => return false,
    };

    file_name.to_string_lossy().starts_with('.')
}

fn filter_dir_entry(entry: &DirEntry, commands: &Commands) -> bool {
    let maximum_depth = 10;

    if entry.depth() >= maximum_depth {
        return false;
    }

    let base_search = entry.file_name().to_str();
    let name_contains = |s: &str| s.contains(&commands.search_for.to_owned());
    match commands.search_by {
        SearchBy::File => base_search.map(name_contains).unwrap_or(false),
        SearchBy::Directory => {
            let is_dir = entry.metadata().unwrap().is_dir();
            let name_match = base_search.map(name_contains).unwrap_or(false);

            is_dir && name_match
        }
        SearchBy::Extension => {
            let mut extension_with_dot = commands.search_for.to_owned();
            if !extension_with_dot.starts_with('.') {
                extension_with_dot = format!(".{}", extension_with_dot);
            }

            base_search
                .map(|s| s.ends_with(&extension_with_dot))
                .unwrap_or(false)
        }
    }
}
