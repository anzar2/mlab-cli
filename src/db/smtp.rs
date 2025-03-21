pub struct SmtpConfig {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub from: String,
    pub from_name: String,
}

impl SmtpConfig {
    pub fn empty() -> SmtpConfig {
        SmtpConfig {
            host: String::from(""),
            port: String::from(""),
            username: String::from(""),
            password: String::from(""),
            from: String::from(""),
            from_name: String::from(""),
        }
    }

    pub fn new(
        host: String,
        port: String,
        username: String,
        password: String,
        from: String,
        from_name: String,
    ) -> SmtpConfig {
        SmtpConfig {
            host,
            port,
            username,
            password,
            from,
            from_name,
        }
    }
}

