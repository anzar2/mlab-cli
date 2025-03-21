use crate::{console, db::user::UserConfig};
use std::process::Command;
pub struct Artisan;
impl ArtisanCommand for Artisan {}

impl Artisan {
    pub fn migrate() {
        Self::cmd("Migrating database...", "php", vec!["artisan", "migrate", "--force"]);
    }

    pub fn seed() {
        Self::cmd("Seeding database...", "php", vec!["artisan", "db:seed"]);
    }

    pub fn generate_key() {
        Self::cmd("Generating key...", "php", vec!["artisan", "key:generate"]);
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
        Self::cmd("Creating team...", "php", vec!["artisan", "app:create-team", team_name]);
    }
}

trait ArtisanCommand {
    fn cmd(message: &str, command: &str, args: Vec<&str>) {
        console::info(message);
        let output = Command::new(command).args(args).output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if !stdout.is_empty() {
                    console::print(stdout.to_string().as_str());
                    console::success("Ok");
                }
            }
            Err(e) => {
                console::error(e.to_string().as_str());
            }
        }
    }
}
