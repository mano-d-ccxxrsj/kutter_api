use crate::adapters::types::DefaultConfig;

impl DefaultConfig {
    pub const DEFAULT_APP_HOST: &str = "127.0.0.1";
    pub const DEFAULT_CLIENT_PORT: u16 = 3001;
    pub const DEFAULT_APP_PORT: u16 = 8080;
    pub const DEFAULT_DB_HOST: &str = "localhost";
    pub const DEFAULT_DB_DOCKER_HOST: &str = "db_postgres";
    pub const DEFAULT_DB_USER: &str = "postgres";
    pub const DEFAULT_DB_PASSWORD: &str = "postgres";
    pub const DEFAULT_DB_NAME: &str = "kutter_db";
    pub const DEFAULT_PORT: u16 = 5432;
    pub const ACQUIRE_TIMEOUT: u64 = 5;
    pub const DEFAULT_MAX_CONNECTIONS: u32 = 20;
    pub const DEFAULT_MIN_CONNECTIONS: u32 = 5;
    pub const DEFAULT_MAX_LIFETIME: u64 = 60 * 60;
    pub const DEFAULT_IDLE_TIMEOUT: u64 = 30 * 60;
    pub const DEFAULT_USE_HTTPS: bool = false;
    pub const DEFAULT_CONTENT_MODERATION_ENABLED: bool = false;
}