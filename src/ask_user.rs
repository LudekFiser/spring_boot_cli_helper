use std::error::Error;
use dialoguer::Select;

pub(crate) enum DbChoice {
    Postgres,
    Mysql,
}

pub(crate) fn ask_user_for_db() -> Result<DbChoice, Box<dyn Error>> {
    let items = vec!["[1] - PostgreSQL", "[2] - MySQL"];

    let selection = Select::new()
        .with_prompt("Choose a Database")
        .items(&items)
        .default(0)
        .interact()?;

    let result = match selection {
        0 => DbChoice::Postgres,
        1 => DbChoice::Mysql,
        _ => unreachable!()
    };

    Ok(result)
}

pub(crate) enum FlywayScripts {
    Yes,
    No
}

pub(crate) fn ask_user_create_flyway_scripts() -> Result<FlywayScripts, Box<dyn Error>> {
    let items = vec!["[1] - Yes", "[2] - No"];

    let selection = Select::new()
        .with_prompt("Do you want to create Flyway Scripts?")
        .items(&items)
        .default(0)
        .interact()?;

    let result = match selection {
        0 => FlywayScripts::Yes,
        1 => FlywayScripts::No,
        _ => unreachable!()
    };

    Ok(result)
}

