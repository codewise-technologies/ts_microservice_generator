pub mod commands;
pub mod utils;

pub fn run() {
    commands::generate::generate_project();
}
