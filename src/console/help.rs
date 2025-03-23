use crate::console;
use crate::utils::TrimIdentation;
use crate::config::Commands;

pub fn print_help() {
    console::header("Help", "");
    println!("{}", format!(
            r#"{}
            mlab [command]

            {}
            [{}] => Show this text
            [{}] => Install MiceLab
            [{}] => Change the app to debug mode
            [{}] => Change the app to production mode
            [{}] => Check if app is on production or debug,
            [{}] => Uninstall MiceLab
            [{}] => Show MiceLab version
            "#,
            console::styles::bold("Usage:"),
            console::styles::bold("Available Commands:"),
            Commands::HELP,
            Commands::INSTALL,
            Commands::ENV_DEBUG,
            Commands::ENV_PRODUCTION,
            Commands::ENV_CHECK,
            Commands::UNINSTALL,
            Commands::VERSION
        ).trim_indentation()); 
}
