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

            write_into_yml_file().expect("Failed to write to file");
            write_into_env_file().expect("Failed to write to env file");
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
fn write_into_yml_file() -> Result<(), Box<dyn Error>> {
    let mut file = get_yml_file()?;
    let chosen_db = ask_user_for_db();

    let yml = generate_yml(chosen_db);
    file.write_all(yml.as_bytes())?;
    Ok(())
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

fn generate_yml(chosen_db: DbChoice) -> String {
    let (url, driver) = match chosen_db {
        DbChoice::Postgres =>
            ("jdbc:postgresql://localhost:5432/${DB_NAME}".to_string(),
            "org.postgresql.Driver".to_string()),
        DbChoice::Mysql =>
            ("jdbc:mysql://localhost:3306/${DB_NAME}".to_string(),
             "com.mysql.cj.jdbc.Driver".to_string()),
    };

    let yml_content = format!(
"spring:
  datasource:
    url: {url}
    username: ${{DB_USERNAME}}
    password: ${{DB_PASSWORD}}
    driver-class-name: {driver}

  jpa:
    hibernate:
      ddl-auto: update
    show-sql: true
    properties:
      hibernate:
        format_sql: true
");

    yml_content
}


enum DbChoice {
    Postgres,
    Mysql,
}

fn ask_user_for_db()  -> DbChoice {
    let items = vec!["[1] - PostgreSQL (Default)", "[2] - MySQL"];


    let selection = Select::new()
        .with_prompt("Choose a Database")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => DbChoice::Postgres,
        1 => DbChoice::Mysql,
        _ => unreachable!()
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

fn create_env_file() -> Result<File, Box<dyn Error>> {
    let file = File::create(".env")?;
    Ok(file)
}

fn write_into_env_file() -> Result<(), Box<dyn Error>> {
    let file = create_env_file();
    let text = "DB_NAME=\nDB_USERNAME=\nDB_PASSWORD=";
    file?.write_all(text.as_bytes())?;
    Ok(())
}
