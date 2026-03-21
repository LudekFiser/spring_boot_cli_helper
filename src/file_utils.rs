use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
// TODO add another function validating the filename, so that we find the right one for each case, idk
pub(crate) fn find_file_by_name(root: &str, filename: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    if filename.is_empty() {
        Err("File name is empty.".into())
    } else {
        Ok(WalkDir::new(root)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_name().to_string_lossy().ends_with(filename))
            .map(|entry| entry.path().to_owned())
            .collect())
    }
}

pub(crate) fn create_folders(path: &Path, choice: PathChoice) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        return Err(Box::from("Path is empty!"));
    }
    let chosen_folder = match choice {
        PathChoice::Main => vec![
            "config", "controller", "repository", "entity",
            "mapper", "service", "exception", "service/impl"],
        PathChoice::Resources => vec!["db", "db/migration"],
    };
    for folder in chosen_folder {
        let name = path.join(folder);
        if !name.exists() {
            fs::create_dir(name)?;
        }
    }
    Ok(())
}

pub(crate) enum PathChoice {
    Main,
    Resources
}