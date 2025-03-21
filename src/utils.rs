use std::path::Path;
use std::process::Command;

// This validation should be more robust on next releases, but for now it is enough for me
// - anzar02
pub fn validate_environment() -> Result<(), String> {
    if Command::new("php").arg("-v").output().is_err() {
        return Err("You need to have PHP installed to use this tool".to_string());
    }

    if !Path::new("artisan").exists() {
        return Err("You need to be in a Laravel project to use this tool".to_string());
    }

    return Ok(());
}


pub fn dotenv_exists() -> bool {
    return Path::new(".env").exists();
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