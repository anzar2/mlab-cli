mod artisan;
mod config;
mod console;
mod db;
mod installer;
mod ui;
mod utils;
mod validators;
mod test;

use installer::Installer;
use artisan::Artisan;
use config::Commands;
use std::env;

fn main() {
    let mut installer = Installer::init();
    
    match env::args().nth(1).as_deref() {
        Some(Commands::INSTALL)         => installer.run(),
        Some(Commands::HELP)            => console::help::print_help(),
        Some(Commands::ENV_DEBUG)       => Artisan::set_development(),
        Some(Commands::ENV_PRODUCTION)  => Artisan::set_production(),
        Some(Commands::UNINSTALL)       => Artisan::uninstall(),
        Some(Commands::ENV_CHECK)       => Artisan::check_environment(),
        
        // Only available in debug mode
        #[cfg(debug_assertions)]
        Some(Commands::TEST_BASIC_INSTALLATION) => test::test_basic_installation(),

        _ => {
            console::help::print_help();
            std::process::exit(1);
        }
    }
}
