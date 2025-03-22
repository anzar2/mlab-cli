use std::path::Path;
use std::process::Command;

// This validation should be more robust on next releases, but for now it is enough for me
// - anzar2
pub fn validate_environment() -> Result<(), String> {
    if Command::new("php").arg("-v").output().is_err() {
        return Err("You need to have PHP installed to use this tool".to_string());
    }

    if !Path::new("artisan").exists() {
        return Err(
            "[Denied] Make sure you are in a valid micelab project\n\
            [Reason] 'artisan' file not found.".to_string());
    }

    if !Path::new("node_modules").exists() {
        return Err(
            "[Denied] Make sure you are in a valid micelab project\n\
            [Reason] 'node_modules/' folder not found.\n\
            [Tip] nodejs modules may not be installed. try running 'npm install' (npm required)".to_string());
    }

    if !Path::new("vendor/").exists() {
        return Err(
            "[Denied] Make sure you are in a valid micelab project\n\
            [Reason] vendor/laravel/ folder not found\n\
            [Tip] Laravel may not be installed. try running 'composer install' (composer required)\n\
            ".to_string()
        );
    }

    return Ok(());
}
pub trait TrimIdentation {
    fn trim_indentation(&self) -> String;
}
impl TrimIdentation for String {
    fn trim_indentation(&self) -> String {
        return self
            .lines()
            .map(|line| line.trim_start())
            .collect::<Vec<&str>>()
            .join("\n");
    }
}
