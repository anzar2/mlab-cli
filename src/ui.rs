use crate::validators::{email, required, validate_port};
use crate::{config, console};
use crate::{db::database::DatabaseConfig, db::smtp::SmtpConfig, db::user::UserConfig};
use figlet_rs::FIGfont;
use unidecode;

use inquire::{Confirm, Password, PasswordDisplayMode::Masked, Select, Text};

fn print_title() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("MiceLab");
    assert!(figure.is_some());
    let title: String = console::styles::purple(figure.unwrap().to_string().as_str());
    println!("{title}");
}

fn print_description() {
    let description: String = format!(
        "{}\n{}\n{}\n{}\n",
        console::styles::bold("Welcome to MiceLab installer"),
        "This installer will create a .env file and make the neccesary migrations\n",
        format!(
            "{}: {}",
            console::styles::bold("Repository"),
            "https://github.com/anzar2/micelab"
        ),
        format!(
            "{}: {}",
            console::styles::bold("Created by"),
            "https://github.com/anzar2"
        )
    );
    println!("{description}");
}

pub fn print_welcome() {
    console::clean();
    print_title();
    print_description();
}

// Installer UI
pub fn select_database() -> DatabaseConfig {
    console::header(
        "Database configuration",
        "[sqlite]:\tSQLite\n[mysql]:\tMySQL\n[mariadb]:\tMariaDB",
    );
    let supported_db = config::supported_databases();

    let db_engine = Select::new("Select database engine:\n", supported_db)
        .with_help_message("The connection will be tested after entering the information")
        .without_filtering()
        .prompt()
        .expect("Error: Database engine is required")
        .to_string();

    if db_engine == "sqlite" {
        return DatabaseConfig::sqlite();
    }

    let db_host = Text::new("Host:")
        .with_placeholder("localhost")
        .with_validator(required)
        .prompt()
        .expect("Error: Database host is required");

    let db_port = Text::new("Port:")
        .with_placeholder("3306")
        .with_validators(&[Box::new(validate_port), Box::new(required)])
        .prompt()
        .expect("Error: Database port is required");

    let db_username = Text::new("Username:")
        .with_placeholder("root")
        .with_validator(required)
        .prompt()
        .expect("Error: Database username is required");

    let db_password = Password::new("Password:")
        .with_validator(required)
        .with_display_mode(Masked)
        .prompt()
        .expect("Error: Database password is required");

    let db_name = Text::new("Database name:")
        .with_validator(required)
        .with_placeholder("micelab")
        .prompt()
        .expect("Error: Database name is required");

    return DatabaseConfig::new(
        db_engine,
        db_host,
        db_port,
        db_username,
        db_password,
        db_name,
    );
}

pub fn select_locale() -> String {
    console::header(
        "Default language configuration",
        "The default language will be used in the application. Users can change it in the settings later",
    );
    let locale = Select::new("Select lang:\nsearch:", config::supported_languages())
        .with_help_message("Type to search")
        .prompt()
        .unwrap()
        .to_string();
    return locale;
}

pub fn ask_insert() -> bool {
    console::header("Example data insertion", "");
    let insert = Confirm::new("Do you want to insert example data? (default: no)")
        .with_default(false)
        .with_help_message("Two users, a project, tasks, test cases, and comments")
        .prompt();
    return insert.unwrap();
}

pub fn create_user() -> UserConfig {
    console::header(
        "Admin user configuration",
        "Please configure your admin user (you will use this user to login after installation)",
    );

    let first_name = Text::new("First name:")
        .with_placeholder("John")
        .with_validator(required)
        .prompt()
        .expect(" Error: First name is required")
        .trim()
        .to_string();

    let last_name = Text::new("Last name:")
        .with_placeholder("Doe")
        .with_validator(required)
        .prompt()
        .expect("Error: Last name is required")
        .trim()
        .to_string();

    let uname_placeholder = unidecode::unidecode(
        format!(
            "{}.{}",
            &first_name[0..3].to_lowercase(),
            last_name.to_lowercase()
        )
        .as_str(),
    );

    let mut username = Text::new("Username:")
        .with_placeholder(&uname_placeholder)
        .with_help_message("Leave blank to use the same username as the placeholder")
        .prompt()
        .expect("Error creatubg username")
        .trim()
        .to_string();

    let email = Text::new("Email:")
        .with_placeholder("o2bNt@example.com")
        .with_validators(&[Box::new(required), Box::new(email)])
        .prompt()
        .expect("Error: Email is required")
        .trim()
        .to_string();

    let password = Password::new("Password:")
        .with_display_mode(Masked)
        .with_validator(required)
        .prompt()
        .expect("Error: Password is required");

    username = if username.is_empty() { uname_placeholder } else { username };

    return UserConfig::new(first_name, last_name, email, username, password);
}

pub fn create_team() -> String {
    console::header("Team name configuration", "Configure your team name");
    let team_name = Text::new("Team name:")
        .with_validator(required)
        .with_placeholder("My Team")
        .prompt()
        .expect("Error: Team name is required")
        .trim()
        .to_string();
    return team_name;
}

pub fn configure_smtp() -> SmtpConfig {
    console::header("SMTP configuration", "Configure your SMTP server");

    let ask_smtp = Confirm::new("Do you want to configure SMTP? (default: yes)")
        .with_default(true)
        .with_help_message("SMTP is a mail transfer protocol, used for sending emails.\nYou can configure it later in .env file or use a different driver.\nMeanwhile, if you don't configure SMTP you won't be able to use features such as recovery password")
        .prompt().unwrap();

    if !ask_smtp {
        return SmtpConfig::empty();
    }

    let smtp_host = Text::new("Host:")
        .with_placeholder("smtp.example.com")
        .with_validator(required)
        .prompt()
        .expect("Error: SMTP host is required")
        .trim()
        .to_string();

    let smtp_port = Text::new("Port:")
        .with_placeholder("587")
        .with_validators(&[Box::new(required), Box::new(validate_port)])
        .prompt()
        .expect("Error: SMTP port is required")
        .trim()
        .to_string();

    let smtp_username = Text::new("Username:")
        .with_placeholder("username")
        .with_validator(required)
        .prompt()
        .expect("Error: SMTP username is required")
        .trim()
        .to_string();

    let smtp_password = Password::new("Password:")
        .with_display_mode(Masked)
        .prompt()
        .expect("Error: SMTP password is required");

    let smtp_from = Text::new("From:")
        .with_placeholder("7o9pD@example.com")
        .with_validator(required)
        .with_help_message("This is the email address that will be used to send emails")
        .prompt()
        .expect("Error: SMTP from is required")
        .trim()
        .to_string();

    let smtp_from_name = Text::new("Mail from:")
        .with_placeholder("Name")
        .with_help_message("This is the name it will show in the email as a sender. Leave it in blank to use your Team name")
        .prompt()
        .expect("Error while getting SMTP from name").trim().to_string();

    return SmtpConfig::new(
        smtp_host,
        smtp_port,
        smtp_username,
        smtp_password,
        smtp_from,
        smtp_from_name,
    );
}


pub fn ask_proceed() -> bool{
    let proceed = Confirm::new("Do you want to proceed?")
        .with_default(false)
        .prompt().expect("Error: an error occurred on asking proceed");

    return proceed;
}