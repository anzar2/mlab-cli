/**
 * ONLY ON DEBUG MODE
 *
 * Description: It does a basic installation using SQLite with SMTP
 * Type: Functional
 * Expected behaviour:
 *      1. Installer runs without errors and exit codes
 */
#[cfg(debug_assertions)]
pub fn test_basic_installation() {
    use crate::artisan::Artisan;
    use crate::artisan::ArtisanCommand;

    crate::console::header("Running basic installation...", "Please wait");
    if let Err(error) = crate::utils::validate_environment() {
        crate::console::error(&error);
        std::process::exit(1);
    }

    let mut installer = crate::installer::Installer::init();

    installer.database_config = crate::db::database::DatabaseConfig::sqlite();
    installer.team_name = String::from("TEST");
    installer.locale = "en".to_string();
    installer.insert_example_data = true;

    installer.user_config = crate::db::user::UserConfig::new(
        String::from("TEST"),
        String::from("TEST"),
        String::from("TEST"),
        String::from("TEST"),
        String::from("TEST"),
    );

    installer.smpt_config = crate::db::smtp::SmtpConfig::new(
        String::from("TEST"),
        String::from("TEST"),
        String::from("TEST"),
        String::from("TEST"),
        String::from("TEST"),
        String::from("TEST"),
    );
    installer.install();

    crate::console::header("Installation simulation finished", "Expected behaviour:\n\n1. Installer runs without errors and exit codes");
    crate::console::print("Result:");
    crate::console::success("Installation ran without errors");
    
    crate::console::info("Cleaning...");
    Artisan::cmd(
        "Executing 'php artisan db:wipe'...",
        "php",
        vec!["artisan", "db:wipe", "--force"],
    );

    if std::path::Path::new(".env").exists() {
        std::fs::remove_file(".env").expect("Unable to remove .env file.");
        crate::console::success("Removed .env file");
    }
}
