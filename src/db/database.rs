use crate::console;
use mysql;
use rusqlite;
pub struct DatabaseConfig {
    pub engine: String,
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn new(
        engine: String,
        host: String,
        port: String,
        username: String,
        password: String,
        database: String,
    ) -> DatabaseConfig {
        DatabaseConfig {
            engine,
            host,
            port,
            username,
            password,
            database,
        }
    }
    pub fn empty() -> DatabaseConfig {
        DatabaseConfig {
            engine: "".to_string(),
            host: "".to_string(),
            port: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            database: "".to_string(),
        }
    }

    pub fn sqlite() -> DatabaseConfig {
        DatabaseConfig {
            engine: "sqlite".to_string(),
            host: "".to_string(),
            port: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            database: "database/database.sqlite".to_string(),
        }
    }

    pub fn ping(&self) -> Result<(), String> {
        match self.engine.as_str() {
            "sqlite" => match rusqlite::Connection::open(&self.database) {
                Ok(_) => Ok(()),
                Err(error) => Err(error.to_string()),
            },
            "mysql" | "mariadb" => {
                let url = format!(
                    "mysql://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, self.port, self.database
                );
                println!("{}", console::styles::cyan("Stablishing connection to database..."));
                match mysql::Pool::new(url.as_str()) {
                    Ok(_) => Ok(()),
                    Err(error) => Err(error.to_string()),
                }
            },
            // TODO
            // "pgsql" => {}
            // "sqlsrv" => {}
            &_ => {
                return Err("Couldn't stablish connection to database".to_string());
            }
        }
    }
}
