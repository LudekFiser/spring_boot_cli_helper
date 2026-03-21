mod db;
mod yml;
mod env;
mod file_utils;

fn main() {
    //TODO instead of find_file_by_name use Path/PathBuff
    let paths = file_utils::find_file_by_name("./", "Application.java");
    if !paths.is_empty() {
        for path in paths {
            let path_without_suffix = path.parent().unwrap();
            file_utils::create_folders(path_without_suffix).expect("Failed to create folders");

            yml::write_into_yml_file().expect("Failed to write to file");
            env::write_into_env_file().expect("Failed to write to env file");
        }
    }
}