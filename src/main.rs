use std::error::Error;
use std::fs;
use std::fs::{File};
use std::io::{Write};
use std::path::{Path, PathBuf};
use walkdir::{WalkDir};
use dialoguer::Select;
fn main() {
    //TODO instead of find_file_by_name use Path/PathBuff
    let paths = find_file_by_name("./", "Application.java");
    if !paths.is_empty() {
        for path in paths {
            let path_without_suffix = path.parent().unwrap();
            create_folders(path_without_suffix).expect("Failed to create folders");

            write_into_yml_file();


        }
    }
}

fn get_yml_file() -> Result<File, Box<dyn Error>> {
    //TODO instead of find_file_by_name use Path/PathBuff
    let yml_file = find_file_by_name("./", "application.yml");
    if !yml_file.is_empty() {
        let stringed = &yml_file[0];
        Ok(File::create(&stringed).expect(&format!("Failed to open file: {:?}", stringed)))
    } else {
        let properties_file = find_file_by_name("./", "application.properties");
        if !properties_file.is_empty() {
            let stringed = properties_file[0].clone();
            rename_file_if_properties(stringed)
        } else {
            Err("Failed to find file to write to".into())
        }
    }
}

fn rename_file_if_properties(properties: PathBuf) -> Result<File, Box<dyn Error>> {
    if properties.exists() {
        let new_path = properties.with_file_name("application.yml");
        fs::rename(&properties, &new_path)?;
        Ok(File::create(&new_path)?)
    } else {
        Err(Box::from("Failed to find application.yml"))
    }
}

fn write_into_yml_file() {
    let mut file = get_yml_file().unwrap();
    let chosen_db = ask_user_for_db();

    let url: String;
    if chosen_db == "PG" {
        url = "jdbc:postgresql://localhost:5432/${DB_NAME}".to_string();
    } else {
        url = "jdbc:mysql://localhost:3306/${DB_NAME}".to_string();
    }

    let yml_content = format!("spring:\n\
                                \tdatasource:\n\
                                    \t\turl: {url}");
    file.write_all(yml_content.as_ref()).expect("gaga")
}

// TODO add enums instead of vec mf
fn ask_user_for_db()  -> &'static str {
    let items = vec!["[1] - PostgreSQL (Default)", "[2] - MySQL"];

    let selection = Select::new()
        .with_prompt("Choose a Database")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();
    let choice = &selection;
    if *choice == 0 {
        "PG"
    } else {
        "MSQL"
    }
}



// find main.java to get the path
fn find_file_by_name(root: &str,  filename: &str) -> Vec<PathBuf> {
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

// create folders
fn create_folders(root: &Path) -> Result<&str, Box<dyn Error>> {
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

