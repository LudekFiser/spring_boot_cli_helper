use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub(crate) fn find_file_by_name(root: &str, filename: &str) -> Vec<PathBuf> {
    if filename.is_empty() {
        panic!("File name is empty");
    } else {
        WalkDir::new(root)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_name().to_string_lossy().ends_with(filename))
            .map(|entry| entry.path().to_owned())
            .collect()
    }
}

pub(crate) fn create_folders(root: &Path) -> Result<&str, Box<dyn Error>> {
    let folders_to_create = vec![
        "config", "controller", "repository", "entity",
        "mapper", "service", "exception", "service/impl"];
    if !root.exists() {
        return Err(Box::from("Path is empty!"));
    }

    for folder in folders_to_create {
        let name = Path::new(root).join(folder);
        if !name.exists() {
            fs::create_dir(name)?;
        }
    }

    Ok("Folders Successfully created!")
}