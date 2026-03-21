use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use crate::file_utils;
use crate::db;
use crate::db::DbChoice;

fn get_yml_file() -> Result<File, Box<dyn Error>> {
    //TODO instead of find_file_by_name use Path/PathBuff
    let yml_file = file_utils::find_file_by_name("./", "application.yml");
    if !yml_file.is_empty() {
        let stringed = &yml_file[0];
        Ok(File::create(&stringed).expect(&format!("Failed to open file: {:?}", stringed)))
    } else {
        let properties_file = file_utils::find_file_by_name("./", "application.properties");
        if !properties_file.is_empty() {
            let stringed = properties_file[0].clone();
            rename_file_if_properties(stringed)
        } else {
            Err("Failed to find file to write to".into())
        }
    }
}

pub(crate) fn write_into_yml_file() -> Result<(), Box<dyn Error>> {
    let mut file = get_yml_file()?;
    let chosen_db = db::ask_user_for_db();

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