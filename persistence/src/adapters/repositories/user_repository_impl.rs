use async_trait::async_trait;
use domain::entities::user_entity::{NewUser, User};
use domain::ports::repositories::user_repository_port::UserRepositoryPort;
use shared::errors::repository_error::RepositoryError;
use sqlx::postgres::PgQueryResult;

use crate::database::types::{PoolWrapper, PostgresUserRepository};
use crate::mappers::user_mapper::UserMapper;
use crate::models::user_model::UserModel;

impl PostgresUserRepository {
    pub fn new(pool: PoolWrapper) -> PostgresUserRepository {
        PostgresUserRepository { pool }
    }
}

#[async_trait]
impl UserRepositoryPort for PostgresUserRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, RepositoryError> {
        let model: Option<UserModel> = sqlx::query_as::<_, UserModel>(
            "SELECT id, username, email, password_hash, public_key, verified,
                    profile_picture, biography, created_at, updated_at
             FROM users
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool.inner)
        .await
        .map_err(|error| RepositoryError::new(error.to_string()))?;

        match model {
            Some(user) => UserMapper::from_model(user).map(Some),
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        let model: Option<UserModel> = sqlx::query_as::<_, UserModel>(
            "SELECT id, username, email, password_hash, public_key, verified,
                    profile_picture, biography, created_at, updated_at
             FROM users
             WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool.inner)
        .await
        .map_err(|error| RepositoryError::new(error.to_string()))?;

        match model {
            Some(user) => UserMapper::from_model(user).map(Some),
            None => Ok(None),
        }
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError> {
        let model: Option<UserModel> = sqlx::query_as::<_, UserModel>(
            "SELECT id, username, email, password_hash, public_key, verified,
                    profile_picture, biography, created_at, updated_at
             FROM users
             WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool.inner)
        .await
        .map_err(|error| RepositoryError::new(error.to_string()))?;

        match model {
            Some(user) => UserMapper::from_model(user).map(Some),
            None => Ok(None),
        }
    }

    async fn create(&self, user: &NewUser) -> Result<(), RepositoryError> {
        let _: PgQueryResult = sqlx::query(
            "INSERT INTO users (
                username, email, password_hash, public_key, verified,
                profile_picture, biography, created_at, updated_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )",
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(user.public_key.as_slice())
        .bind(user.verified)
        .bind(&user.profile_picture)
        .bind(&user.biography)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool.inner)
        .await
        .map_err(|error| RepositoryError::new(error.to_string()))?;

        Ok(())
    }
}