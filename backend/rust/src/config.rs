use std::env;

const DEFAULT_DB_MAX_CONNECTIONS: u32 = 10;
const DEFAULT_FRONTEND_DIR: &str = "../../frontend";
const DEFAULT_PORT: u16 = 3000;

pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub db_max_connections: u32,
    pub frontend_dir: String,
}

impl Config {
    pub fn from_env() -> Self {
        let db_host = env::var("TESTFLIX_DB_HOST").expect("TESTFLIX_DB_HOST must be set");
        let db_port = env::var("TESTFLIX_DB_PORT")
            .expect("TESTFLIX_DB_PORT must be set")
            .parse::<u16>()
            .expect("TESTFLIX_DB_PORT must be a valid port number");
        let db_name = env::var("TESTFLIX_DB_NAME").expect("TESTFLIX_DB_NAME must be set");
        let db_user = env::var("TESTFLIX_DB_USER").expect("TESTFLIX_DB_USER must be set");
        let db_password =
            env::var("TESTFLIX_DB_PASSWORD").expect("TESTFLIX_DB_PASSWORD must be set");

        let database_url = format!(
            "postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}"
        );

        let port = env::var("TESTFLIX_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(DEFAULT_PORT);

        let db_max_connections = env::var("DB_MAX_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(DEFAULT_DB_MAX_CONNECTIONS);

        let frontend_dir =
            env::var("FRONTEND_DIR").unwrap_or_else(|_| DEFAULT_FRONTEND_DIR.to_string());

        Self {
            database_url,
            port,
            db_max_connections,
            frontend_dir,
        }
    }
}
