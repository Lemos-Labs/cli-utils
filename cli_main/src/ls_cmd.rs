use std::{fs, io, path::PathBuf};

pub fn get_current_dir_entries(path: &String) -> Result<Vec<PathBuf>, io::Error> {
    let entries = fs::read_dir(path)?;

    let mut paths: Vec<PathBuf> = Vec::new();

    for entry in entries {
        let entry = entry?;
        paths.push(entry.path());
    };
    
    Ok(paths)
} 