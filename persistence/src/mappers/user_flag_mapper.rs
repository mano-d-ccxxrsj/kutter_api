use domain::entities::user_flag_entity::UserFlag;

use crate::models::user_flag_model::UserFlagModel;

pub struct UserFlagMapper;

impl UserFlagMapper {
    pub fn from_model(model: UserFlagModel) -> UserFlag {
        UserFlag {
            id: model.id,
            user_id: model.user_id,
            field: model.field,
            action: model.action,
            target: model.target,
            attempted_text: model.attempted_text,
            matched_words: model.matched_words,
            details: model.details,
            created_at: model.created_at,
        }
    }
}