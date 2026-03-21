use std::error::Error;
use std::path::{PathBuf};
use dialoguer::MultiSelect;
use crate::{env, file_utils, yml};

pub(crate) fn setup() -> Result<(), Box<dyn Error>> {

    let mut choices: Vec<Choices> = Vec::new();
    let items = vec![
        "[1] - Create Folders", "[2] - Setup .yml file",
        "[3] - Create .env file"];

    let selection = MultiSelect::new()
        .with_prompt("Choose all you wanna do")
        .items(&items)
        .interact()?;
    for selected in selection {
        match selected {
            0 => choices.push(Choices::CreateFolders),
            1 => choices.push(Choices::SetupYmlFile),
            2 => choices.push(Choices::CreateEnvFile),
            _ => unreachable!(),
        }
    }
    for choice in choices {
        match choice {
            Choices::CreateFolders => file_utils::create_folders(&get_main_path_without_suffix()?)?,
            Choices::SetupYmlFile => yml::write_into_yml_file()?,
            Choices::CreateEnvFile => env::write_into_env_file()?,
        }
    }

    Ok(())
}

enum Choices {
    CreateFolders,
    SetupYmlFile,
    CreateEnvFile
}

fn get_main_path_without_suffix() -> Result<PathBuf, Box<dyn Error>> {
    let paths = file_utils::find_file_by_name("./", "Application.java")?;
    let path = paths.first().ok_or("Failed to get path")?;
    let path_without_suffix = path.parent().ok_or("Failed to get parent directory")?;
    Ok(path_without_suffix.to_path_buf())

}


