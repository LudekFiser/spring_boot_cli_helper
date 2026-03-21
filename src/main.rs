use std::error::Error;

mod db;
mod yml;
mod env;
mod file_utils;

fn main() {
    setup().expect("Failed to setup project");
}


fn setup() -> Result<(), Box<dyn Error>> {
    let paths = file_utils::find_file_by_name("./", "Application.java")?;
    for path in paths {
        let path_without_suffix = path.parent().ok_or("Failed to get parent directory")?;
        file_utils::create_folders(path_without_suffix)?;

        yml::write_into_yml_file()?;
        env::write_into_env_file()?;
    }
    Ok(())

}