use crate::adapters::types::{DefaultConfig, EnvConfig};
use shared::config::ports::ConfigPort;
use shared::config::types::AppConfig;
use std::{env, path::Path};

impl ConfigPort for EnvConfig {
    fn from_env_file() -> AppConfig {
        let _ = dotenvy::dotenv();

        let mut db_host: String = env::var("DB_HOST")
            .map(|v: String| v.trim().to_string())
            .unwrap_or_else(|_: env::VarError| DefaultConfig::DEFAULT_DB_HOST.to_string());

        let mut app_host: String = env::var("APP_HOST")
            .map(|v: String| v.trim().to_string())
            .unwrap_or_else(|_: env::VarError| DefaultConfig::DEFAULT_APP_HOST.to_string());

        if Path::new("/.dockerenv").exists() {
            if db_host == DefaultConfig::DEFAULT_DB_HOST || db_host == "127.0.0.1" {
                db_host = DefaultConfig::DEFAULT_DB_DOCKER_HOST.to_string();
            }
            app_host = "0.0.0.0".to_string();
        }

        let db_user: String = env::var("DB_USER")
            .map(|v: String| v.trim().to_string())
            .unwrap_or_else(|_: env::VarError| DefaultConfig::DEFAULT_DB_USER.to_string());

        let db_password: String = env::var("DB_PASSWORD")
            .map(|v: String| v.trim().to_string())
            .unwrap_or_else(|_: env::VarError| DefaultConfig::DEFAULT_DB_PASSWORD.to_string());

        let db_name: String = env::var("DB_NAME")
            .map(|v: String| v.trim().to_string())
            .unwrap_or_else(|_: env::VarError| DefaultConfig::DEFAULT_DB_NAME.to_string());

        let db_port: u16 = env::var("DB_PORT")
            .map(|v: String| v.trim().to_string()).ok()
            .and_then(|v: String| v.parse::<u16>().ok())
            .unwrap_or(DefaultConfig::DEFAULT_PORT);

        let database_url: String = env::var("DATABASE_URL")
            .map(|v: String| v.trim().to_string())
            .unwrap_or_else(|_: env::VarError| {
                format!(
                    "postgres://{}:{}@{}:{}/{}",
                    db_user, db_password, db_host, db_port, db_name
                )
            });

        AppConfig {
            db_user,
            db_password,
            db_name,
            db_host,
            db_port,
            database_url,
            app_host,

            db_max_connections: env::var("DB_MAX_CONNECTIONS")
                .map(|v: String| v.trim().to_string()).ok()
                .and_then(|v: String| v.parse::<u32>().ok())
                .unwrap_or(DefaultConfig::DEFAULT_MAX_CONNECTIONS),

            db_min_connections: env::var("DB_MIN_CONNECTIONS")
                .map(|v: String| v.trim().to_string()).ok()
                .and_then(|v: String| v.parse::<u32>().ok())
                .unwrap_or(DefaultConfig::DEFAULT_MIN_CONNECTIONS),

            db_acquire_timeout: env::var("DB_ACQUIRE_TIMEOUT")
                .map(|v: String| v.trim().to_string()).ok()
                .and_then(|v: String| v.parse::<u64>().ok())
                .unwrap_or(DefaultConfig::ACQUIRE_TIMEOUT),

            db_max_lifetime: env::var("DB_MAX_LIFETIME")
                .map(|v: String| v.trim().to_string()).ok()
                .and_then(|v: String| v.parse::<u64>().ok())
                .unwrap_or(DefaultConfig::DEFAULT_MAX_LIFETIME),

            db_idle_timeout: env::var("DB_IDLE_TIMEOUT")
                .map(|v: String| v.trim().to_string()).ok()
                .and_then(|v: String| v.parse::<u64>().ok())
                .unwrap_or(DefaultConfig::DEFAULT_IDLE_TIMEOUT),

            app_client_port: env::var("APP_CLIENT_PORT")
                .map(|v: String| v.trim().to_string()).ok()
                .and_then(|v: String| v.parse::<u16>().ok())
                .unwrap_or(DefaultConfig::DEFAULT_CLIENT_PORT),

            app_server_port: env::var("APP_SERVER_PORT")
                .map(|v: String| v.trim().to_string()).ok()
                .and_then(|v: String| v.parse::<u16>().ok())
                .unwrap_or(DefaultConfig::DEFAULT_APP_PORT),

            use_https: env::var("USE_HTTPS")
                .map(|v: String| v.trim().to_string()).ok()
                .and_then(|v: String| v.parse::<bool>().ok())
                .unwrap_or(DefaultConfig::DEFAULT_USE_HTTPS),

            content_moderation_enabled: env::var("CONTENT_MODERATION_ENABLED")
                .map(|v: String| v.trim().to_string()).ok()
                .and_then(|v: String| v.parse::<bool>().ok())
                .unwrap_or(DefaultConfig::DEFAULT_CONTENT_MODERATION_ENABLED),

            jwt_key: env::var("JWT_KEY")
                .map(|v: String| v.trim().to_string())
                .expect("JWT_KEY must be configured"),
        }
    }
}