mod ask_user;
mod yml;
mod env;
mod file_utils;
mod setup;
mod path_utils;

fn main() {
    setup::setup().expect("Failed to setup project");
}
