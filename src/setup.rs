use std::error::Error;
use dialoguer::MultiSelect;
use crate::{env, file_utils, yml, path_utils};
use crate::yml::create_flyway_migration_scripts;

pub(crate) fn setup() -> Result<(), Box<dyn Error>> {

    let mut choices: Vec<Choices> = Vec::new();
    let items = vec![
        "[0] - Do Everything",
        "[1] - Create Main Folders",
        "[2] - Create Flyway Migration Folders",
        "[3] - Setup .yml file",
        "[4] - Create .env file"];

    let selection = MultiSelect::new()
        .with_prompt("Choose all you wanna do")
        .items(&items)
        .interact()?;
    if selection.is_empty() {
        println!("BYE BYE");
        return Ok(())
    }

    if selection.contains(&0) {
        choices.push(Choices::CreateMainFolders);
        choices.push(Choices::CreateResourcesFolders);
        choices.push(Choices::SetupYmlFile);
        choices.push(Choices::CreateEnvFile);
    } else {
        for selected in selection {
            match selected {
                1 => choices.push(Choices::CreateMainFolders),
                2 => choices.push(Choices::CreateResourcesFolders),
                3 => choices.push(Choices::SetupYmlFile),
                4 => choices.push(Choices::CreateEnvFile),
                _ => unreachable!(),
            }
        }
    }
    for choice in choices {
        match choice {
            Choices::CreateMainFolders => file_utils::create_folders(&path_utils::get_main_path()?, file_utils::PathChoice::Main)?,
            Choices::CreateResourcesFolders => {
                file_utils::create_folders(&path_utils::get_resources_path()?, file_utils::PathChoice::Resources)?;
                create_flyway_migration_scripts()?;
            },
            Choices::SetupYmlFile => yml::write_into_yml_file()?,
            Choices::CreateEnvFile => env::write_into_env_file()?
        }
    }
    Ok(())

}

enum Choices {
    CreateMainFolders,
    CreateResourcesFolders,
    SetupYmlFile,
    CreateEnvFile,
}


