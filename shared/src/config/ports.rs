use crate::config::types::AppConfig;

pub trait ConfigPort {
    fn from_env_file() -> AppConfig;
}

pub trait CloneablePort {
    fn clone_config(&self) -> Self;
}