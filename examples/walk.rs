use anyhow::Result;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    let path = entry.path();
    let file_name = match path.file_name() {
        Some(file_name) => file_name,
        None => return false,
    };

    file_name.to_string_lossy().starts_with('.')
}

fn main() -> Result<()> {
    let path = Path::new("..");

    println!("{:?}", &path.canonicalize());

    let wd = WalkDir::new(&path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e));

    for entry in wd.take(5) {
        let ent = entry?;

        // let path = &ent.path();
        // if path.is_dir(){
        //     println!("Path Is Dir: {:?}", path.file_name());
        // }
        // if path.is_file(){
        //     println!("Path Is File: {:?}", path.file_name());
        // }

        println!("Regular: {}", &ent.path().display());

        // let is_dir = &ent.path().is_dir();
    }

    Ok(())
}
