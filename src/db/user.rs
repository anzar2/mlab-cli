pub struct UserConfig {
    pub display_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl UserConfig {
    pub fn new(
        display_name: String,
        email: String,
        username: String,
        password: String,
    ) -> UserConfig {
        UserConfig {
            display_name: String::from(display_name),
            email: String::from(email),
            username: String::from(username),
            password: String::from(password),
        }
    }

    pub fn empty() -> UserConfig {
        UserConfig {
            display_name: String::from(""),
            email: String::from(""),
            username: String::from(""),
            password: String::from(""),
        }
    }
}
