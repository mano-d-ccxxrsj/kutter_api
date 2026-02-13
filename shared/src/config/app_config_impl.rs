use crate::config::types::AppConfig;

impl AppConfig {
    pub fn clone(&self) -> AppConfig {
        AppConfig {
            db_user: self.db_user.clone(),
            db_password: self.db_password.clone(),
            db_host: self.db_host.clone(),
            db_name: self.db_name.clone(),
            db_port: self.db_port,
            database_url: self.database_url.clone(),
            db_max_connections: self.db_max_connections,
            db_min_connections: self.db_min_connections,
            db_max_lifetime: self.db_max_lifetime,
            db_idle_timeout: self.db_idle_timeout,
            db_acquire_timeout: self.db_acquire_timeout,
            app_host: self.app_host.clone(),
            app_client_port: self.app_client_port,
            app_server_port: self.app_server_port,
            use_https: self.use_https,
        }
    }
}