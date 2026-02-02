use std::process::Command;

use crate::settings::{Commands, SearchBy};
use anyhow::Result;

pub fn fd_find(command: &Commands, path: &str) -> Result<Vec<String>> {
    let mut found_files = Vec::new();

    for search_value in &command.search_for {
        let output = Command::new("fd")
            .arg("-i")
            .args(match command.search_by {
                SearchBy::File => ["-t", "f", search_value],
                SearchBy::Directory => ["-t", "d", search_value],
                SearchBy::Extension => ["-e", search_value, "."],
            })
            .arg(path)
            .output()
            .expect("Failed to execute FD. Check if it's installed");

        let resp = String::from_utf8(output.stdout)?;
        let lines: Vec<String> = resp.lines().map(|s| s.to_owned()).collect();
        found_files.extend(lines);
    }

    found_files.sort_by_key(|a| a.to_lowercase());
    found_files.dedup();

    Ok(found_files)
}
