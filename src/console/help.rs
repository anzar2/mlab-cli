use crate::console;
use crate::utils::TrimIdentation;

pub fn set_development() {
    if crate::utils::dotenv_exists() {
        let mut dotenv = std::fs::read_to_string(".env").unwrap();

        if dotenv.contains("APP_DEBUG=true") && dotenv.contains("APP_ENV=local") {
            console::error("Debug mode already enabled");
            return;
        }

        dotenv = dotenv
            .replace("APP_DEBUG=false", "APP_DEBUG=true")
            .replace("APP_ENV=production", "APP_ENV=local");

        match std::fs::write(".env", dotenv) {
            Ok(_) => {
                console::success("Debug mode enabled");
            }
            Err(e) => {
                console::error(e.to_string().as_str());
            }
        };
        return;
    }
    console::error(".env file not found");
}

pub fn set_production() {
    if crate::utils::dotenv_exists() {
        let mut dotenv = std::fs::read_to_string(".env").unwrap();

        if dotenv.contains("APP_DEBUG=false") && dotenv.contains("APP_ENV=production") {
            console::error("Production mode already enabled");
            return;
        }

        dotenv = dotenv
            .replace("APP_DEBUG=true", "APP_DEBUG=false")
            .replace("APP_ENV=local", "APP_ENV=production");

        match std::fs::write(".env", dotenv) {
            Ok(_) => {
                console::success("Production mode enabled");
            }
            Err(e) => {
                console::error(e.to_string().as_str());
            }
        };
        return;
    }
    console::error(".env file not found");
}

pub fn check_environment() {
    match crate::utils::dotenv_exists()
        && std::fs::read_to_string(".env")
            .unwrap()
            .contains("APP_ENV=production")
    {
        true => console::success("The app is in production mode"),
        false => console::success("The app is in development mode"),
    }
}

pub fn print_help() {
    console::header("Help", "");
    println!("{}", format!(
            r#"{}
            mlab [command]

            {}
            help            Show this text
            install         Install MiceLab
            env:debug       Change the app to debug mode
            env:production  Change the app to production mode
            env:check       Check if app is on production or debug
            "#,
            console::styles::bold("Usage:"),
            console::styles::bold("Available Commands:")
        ).trim_indentation()); 
}
