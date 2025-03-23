pub struct Commands;
impl Commands {
    pub const INSTALL: &str = "install";
    pub const HELP : &str = "help";
    pub const ENV_DEBUG : &str = "env:debug";
    pub const ENV_PRODUCTION : &str = "env:production";
    pub const ENV_CHECK : &str = "env:check";
    pub const UNINSTALL : &str = "uninstall";
    pub const VERSION : &str = "version";
    
    #[allow(dead_code)]
    pub const TEST_BASIC_INSTALLATION : &str = "test:basic-installation";
}

pub fn supported_languages() -> Vec<&'static str> {
    return vec!["es", "en"];
}

pub fn supported_databases() -> Vec<&'static str> {
    return vec![
        "sqlite", "mysql", "mariadb",
        // TODO
        // "sqlsrv",
        // "pgsql",
    ];
}

