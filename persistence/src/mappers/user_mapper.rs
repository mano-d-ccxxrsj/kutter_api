use domain::entities::user_entity::User;
use shared::errors::repository_error::RepositoryError;

use crate::models::user_model::UserModel;

pub struct UserMapper;

impl UserMapper {
    pub fn from_model(model: UserModel) -> Result<User, RepositoryError> {
        let public_key: [u8; 32] = model
            .public_key
            .try_into()
            .map_err(|_| RepositoryError::new("Invalid public key length"))?;

        Ok(User {
            id: model.id,
            username: model.username,
            email: model.email,
            password_hash: model.password_hash,
            public_key,
            verified: model.verified,
            profile_picture: model.profile_picture,
            biography: model.biography,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
    }
}