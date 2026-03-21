use std::error::Error;
use dialoguer::MultiSelect;
use crate::{env, file_utils, yml, path_utils};


pub(crate) fn setup() -> Result<(), Box<dyn Error>> {

    let mut choices: Vec<Choices> = Vec::new();
    let items = vec![
        "[1] - Create Main Folders",
        "[2] - Create Flyway Folders",
        "[3] - Setup .yml file",
        "[4] - Create .env file"];

    let selection = MultiSelect::new()
        .with_prompt("Choose all you wanna do")
        .items(&items)
        .interact()?;
    if !selection.is_empty() {
        for selected in selection {
            match selected {
                0 => choices.push(Choices::CreateMainFolders),
                1 => choices.push(Choices::CreateResourcesFolders),
                2 => choices.push(Choices::SetupYmlFile),
                3 => choices.push(Choices::CreateEnvFile),
                _ => unreachable!(),
            }
        }
        for choice in choices {
            match choice {
                Choices::CreateMainFolders => file_utils::create_folders(&path_utils::get_main_path()?, file_utils::PathChoice::Main)?,
                Choices::CreateResourcesFolders => file_utils::create_folders(&path_utils::get_resources_path()?, file_utils::PathChoice::Resources)?,
                Choices::SetupYmlFile => yml::write_into_yml_file()?,
                Choices::CreateEnvFile => env::write_into_env_file()?,
            }
        }

        Ok(())
    } else {
        println!("BYE BYE");
        Ok(())
    }
}

enum Choices {
    CreateMainFolders,
    CreateResourcesFolders,
    SetupYmlFile,
    CreateEnvFile
}


