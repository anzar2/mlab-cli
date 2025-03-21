pub struct UserConfig {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl UserConfig {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        username: String,
        password: String,
    ) -> UserConfig {
        UserConfig {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            email: String::from(email),
            username: String::from(username),
            password: String::from(password),
        }
    }

    pub fn empty() -> UserConfig {
        UserConfig {
            first_name: String::from(""),
            last_name: String::from(""),
            email: String::from(""),
            username: String::from(""),
            password: String::from(""),
        }
    }
}
