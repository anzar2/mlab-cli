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