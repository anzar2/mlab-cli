use crate::console;
use crate::utils::TrimIdentation;
use crate::config::Commands;

pub fn check_environment() {
    match std::fs::read_to_string(".env") {
        Ok(env) => {
            if env.contains("APP_ENV=production") {
                console::success("App is on production mode");
            } else {
                console::success("App is on development mode");
            }
        },
        Err(_) => {
            console::error("Could not read .env file");
        }
    }

}

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
            "#,
            console::styles::bold("Usage:"),
            console::styles::bold("Available Commands:"),
            Commands::HELP,
            Commands::INSTALL,
            Commands::ENV_DEBUG,
            Commands::ENV_PRODUCTION,
            Commands::ENV_CHECK,
            Commands::UNINSTALL
        ).trim_indentation()); 
}
