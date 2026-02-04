use std::{fs, env, str::Lines};

pub struct AppConfig {
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_name: String,
    pub db_port: u16,
    pub database_url: String,
    pub db_max_connections: u32,
    pub db_min_connections: u32,
    pub db_max_lifetime: u64,
    pub db_idle_timeout: u64,
    pub db_acquire_timeout: u64,
    pub app_host: String,
    pub app_client_port: u16,
    pub app_server_port: u16,
    pub use_https: bool,
}

struct DefaultConfig;

impl DefaultConfig {
    const DEFAULT_APP_HOST: &str = "127.0.0.1";
    const DEFAULT_CLIENT_PORT: u16 = 3001;
    const DEFAULT_APP_PORT: u16 = 8080;
    const DEFAULT_PORT: u16 = 8001;
    const ACQUIRE_TIMEOUT: u64 = 5;
    const DEFAULT_MAX_CONNECTIONS: u32 = 20;
    const DEFAULT_MIN_CONNECTIONS: u32 = 5;
    const DEFAULT_MAX_LIFETIME: u64 = 60 * 60;
    const DEFAULT_IDLE_TIMEOUT: u64 = 30 * 60;
    const DEFAULT_USE_HTTPS: bool = false;
}

impl AppConfig {

    pub fn from_env_file() -> AppConfig {
        let content: String = fs::read_to_string(".env").expect("erro lendo .env");

        let mut iter: Lines = content.lines();

        loop {
            let next: Option<&str> = (&mut iter).next();

            if next.is_none() {
                break;
            }

            let line: &str = next.unwrap();

            let trimmed_line: &str = line.trim();

            if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
                continue;
            }

            let parsed: Option<(&str, &str)> = trimmed_line.split_once('=');

            if parsed.is_none() {
                continue;
            }

            let (raw_key, raw_value): (&str, &str) = parsed.unwrap();

            let key: &str = raw_key.trim();
            let value: &str = raw_value.trim();

            unsafe {
                env::set_var(key, value);
            }
        }

        AppConfig::from_env()
    }

    fn from_env() -> AppConfig {
        AppConfig {
            db_user: env::var("DB_USER").unwrap(),
            db_password: env::var("DB_PASSWORD").unwrap(),
            db_host: env::var("DB_HOST").unwrap(),
            db_name: env::var("DB_NAME").unwrap(),

            db_port: env::var("DB_PORT")
                .ok().and_then(|v| v.parse().ok())
                .unwrap_or(DefaultConfig::DEFAULT_PORT),

            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                format!(
                    "postgres://{}:{}@{}:{}/{}",
                    env::var("DB_USER").unwrap(),
                    env::var("DB_PASSWORD").unwrap(),
                    env::var("DB_HOST").unwrap(),
                    env::var("DB_PORT").unwrap(),
                    env::var("DB_NAME").unwrap(),
                )
            }),

            db_max_connections: env::var("DB_MAX_CONNECTIONS")
                .ok().and_then(|v| v.parse().ok())
                .unwrap_or(DefaultConfig::DEFAULT_MAX_CONNECTIONS),

            db_min_connections: env::var("DB_MIN_CONNECTIONS")
                .ok().and_then(|v| v.parse().ok())
                .unwrap_or(DefaultConfig::DEFAULT_MIN_CONNECTIONS),

            db_acquire_timeout: env::var("DB_ACQUIRE_TIMEOUT")
                .ok().and_then(|v| v.parse().ok())
                .unwrap_or(DefaultConfig::ACQUIRE_TIMEOUT),

            db_max_lifetime: env::var("DB_MAX_LIFETIME")
                .ok().and_then(|v| v.parse().ok())
                .unwrap_or(DefaultConfig::DEFAULT_MAX_LIFETIME),

            db_idle_timeout: env::var("DB_IDLE_TIMEOUT")
                .ok().and_then(|v| v.parse().ok())
                .unwrap_or(DefaultConfig::DEFAULT_IDLE_TIMEOUT),

            app_host: env::var("APP_HOST")
                .unwrap_or(DefaultConfig::DEFAULT_APP_HOST.to_string()),

            app_client_port: env::var("APP_CLIENT_PORT")
                .ok().and_then(|v| v.parse().ok())
                .unwrap_or(DefaultConfig::DEFAULT_CLIENT_PORT),

            app_server_port: env::var("APP_SERVER_PORT")
                .ok().and_then(|v| v.parse().ok())
                .unwrap_or(DefaultConfig::DEFAULT_APP_PORT),

            use_https: env::var("USE_HTTPS")
                .ok().and_then(|v| v.parse().ok())
                .unwrap_or(DefaultConfig::DEFAULT_USE_HTTPS),
        }
    }
}