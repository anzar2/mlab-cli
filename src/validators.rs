use std::error::Error;
use inquire::validator::Validation;

pub fn validate_port(input: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
    if input.parse::<i32>().is_err() {
        return Ok(Validation::Invalid("Port must be a number".into()));
    }

    let port = input.parse::<i32>();
    let port = port.unwrap();

    if port > 65535 || port < 1 {
        return Ok(Validation::Invalid(
            "[Error] Port must be between 1 and 65535".into(),
        ));
    }
    if port <= 0 {
        return Ok(Validation::Invalid("[Error] Port can't be <= 0".into()));
    }

    Ok(Validation::Valid)
}


pub fn required(input: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
    if input.is_empty() {
        return Ok(Validation::Invalid("This field is required".into()));
    }
    Ok(Validation::Valid)
}

pub fn email(input: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
    let email_regex = regex::Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").expect("Invalid regex has been passed");
    
    if !email_regex.is_match(input) {
        return Ok(Validation::Invalid("Invalid email format".into()));
    }

    Ok(Validation::Valid)
}