use dialoguer::Select;

pub(crate) enum DbChoice {
    Postgres,
    Mysql,
}

pub(crate) fn ask_user_for_db() -> DbChoice {
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