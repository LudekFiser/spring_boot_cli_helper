use std::error::Error;
use std::path::PathBuf;
use crate::file_utils;

pub(crate) fn get_main_path() -> Result<PathBuf, Box<dyn Error>> {
    let paths = file_utils::find_file_by_name("./", "Application.java")?;
    let path = paths.first().ok_or("Failed to get path")?;
    let path_without_suffix = path.parent().ok_or("Failed to get parent directory")?;
    Ok(path_without_suffix.to_path_buf())
}



pub(crate) fn get_resources_path() -> Result<PathBuf, Box<dyn Error>> {
    let path_yml = file_utils::find_file_by_name("./", "application.yml")?;
    let path;
    if !path_yml.is_empty() {
        path = path_yml.first().ok_or("Failed to get path")?.parent().ok_or("Failed to get parent directory")?;
        Ok(path.to_path_buf())
    } else {
        let path_prop = file_utils::find_file_by_name("./", "application.properties")?;
        if !path_prop.is_empty() {
            path = path_prop.first().ok_or("Failed to get path")?.parent().ok_or("Failed to get parent directory")?;
            Ok(path.to_path_buf())
        } else {
            Err("Failed to get path (Does .yml or .properties exist?)")?
        }
    }
}