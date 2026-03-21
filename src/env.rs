use std::error::Error;
use std::fs::File;
use std::io::Write;

fn create_env_file() -> Result<File, Box<dyn Error>> {
    let file = File::create(".env")?;
    Ok(file)
}

pub(crate) fn write_into_env_file() -> Result<(), Box<dyn Error>> {
    let file = create_env_file();
    let text = "DB_NAME=\nDB_USERNAME=\nDB_PASSWORD=";
    file?.write_all(text.as_bytes())?;
    Ok(())
}