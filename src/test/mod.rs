/**
 * ONLY ON DEBUG MODE
 *
 * Description: It does a basic installation using SQLite with SMTP
 * Type: Functional
 * Expected behaviour:
 *      1. Create the .env file
 *      2. Migrations runs successfully
 *      3. Create the admin user
 *      4. Database seeding runs successfully
 *      5. SMTP on .env has been configured with 'TEST' values
 *      6. Team is set to 'TEST'
 *      7. Locale is set to 'en'
 *
 */
#[cfg(debug_assertions)]
pub fn test_basic_installation() {
    let steps: u8 = 7;
    let mut passed_steps: u8 = 0;

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

    crate::console::clean();
    crate::console::info("Installation simulation finished");
    crate::console::header("Installation verification", "");

    crate::console::print(
        format!(
            "Expected result:\n\
            1. Create the .env file\n\
            2. Migrations runs successfully\n\
            3. Create the admin user\n\
            4. Database seeding runs successfully\n\
            5. SMTP on .env has been configured with 'TEST' values\n\
            6. Team is set to 'TEST'\n\
            7. Locale is set to 'en'\n"
        )
        .as_str(),
    );

    crate::console::info("Results:\n");
    if std::path::Path::new(".env").exists() {
        crate::console::success("Created .env file");
        passed_steps += 1;
    }

    crate::console::print(
        format!(
            "\nSummary: {passed_steps} of {steps} steps passed."
        )
        .as_str(),
    );
}
