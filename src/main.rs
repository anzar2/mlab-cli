mod artisan;
mod config;
mod console;
mod db;
mod installer;
mod ui;
mod utils;
mod validators;

use installer::Installer;
use std::env;

fn main() {
    let mut installer = Installer::init();
    
    match env::args().nth(1).as_deref() {
        Some("install")        => installer.run(),
        Some("help")           => console::help::print_help(),
        Some("env:debug")      => console::help::set_development(),
        Some("env:production") => console::help::set_production(),
        Some("env:check")      => console::help::check_environment(),
        _ => {
            console::help::print_help();
            std::process::exit(1);
        }
    }
}
