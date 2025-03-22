use crate::{console, db::user::UserConfig, ui};
use std::process::Command;
pub struct Artisan;
impl ArtisanCommand for Artisan {}

impl Artisan {
    pub fn migrate() {
        Self::cmd(
            "Migrating database...",
            "php",
            vec!["artisan", "app:migrate-database"],
        );
    }

    pub fn seed() {
        Self::cmd(
            "Seeding database...",
            "php",
            vec!["artisan", "app:seed-database"],
        );
    }

    pub fn generate_key() {
        Self::cmd("Generating key...", "php", vec!["artisan", "key:generate", "--force"]);
    }

    pub fn uninstall() {
        console::warning("All data will be permanently lost. Make sure you have a backup.");
        let ask_proceed = ui::ask_proceed();
        
        if ask_proceed {
            Self::cmd(
                "Executing 'php artisan db:wipe'...",
                "php",
                vec!["artisan", "db:wipe", "--force"],
            );
    
            console::success("Uninstalled successfully");
    
            if std::path::Path::new(".env").exists() {
                std::fs::remove_file(".env").expect("Unable to remove .env file.");
                console::success("Removed .env file");
            }
        }
    }

    pub fn create_user(user_config: &UserConfig) {
        Self::cmd(
            "Creating admin user...",
            "php",
            vec![
                "artisan",
                "app:create-admin",
                user_config.first_name.as_str(),
                user_config.last_name.as_str(),
                user_config.email.as_str(),
                user_config.password.as_str(),
                user_config.username.as_str(),
            ],
        );
    }

    pub fn create_team(team_name: &str) {
        Self::cmd(
            "Creating team...",
            "php",
            vec!["artisan", "app:create-team", team_name],
        );
    }

    pub fn check_installation() {
        Self::cmd(
            "Checking installation...",
            "php",
            vec!["artisan", "app:check-installation"],
        );
    }

    pub fn set_development() {
        console::warning("Is not recommended running on production. It will clear all cache.");
        let ask_proceed = ui::ask_proceed();
        if ask_proceed {
            Self::cmd(
                "Setting development mode...",
                "php",
                vec!["artisan", "app:set-development"],
            );
        }
    }

    pub fn set_production() {
        console::warning("Is not recommended running on production. It will clear all cache.");
        let ask_proceed = ui::ask_proceed();
        if ask_proceed {
            Self::cmd(
                "Setting production mode...",
                "php",
                vec!["artisan", "app:set-production"],
            );
        }
    }
}

pub trait ArtisanCommand {
    fn cmd(message: &str, command: &str, args: Vec<&str>) {
        console::info(message);
        let output = Command::new(command).args(args).output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                if !stdout.is_empty() {
                    console::print(stdout.to_string().as_str());
                    console::success("Ok");
                }

                if !stderr.is_empty() {
                    console::error(stderr.to_string().as_str());
                    std::process::exit(1);
                }
            }
            Err(e) => {
                console::error(e.to_string().as_str());
                std::process::exit(1);
            }
        }
    }
}
