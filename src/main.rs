
mod ask_user;
mod yml;
mod env;
mod file_utils;
mod setup;

fn main() {
    setup::setup().expect("Failed to setup project");
}
