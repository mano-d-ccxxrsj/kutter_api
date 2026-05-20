use domain::entities::banned_word_entity::BannedWord;

use crate::models::banned_word_model::BannedWordModel;

pub struct BannedWordMapper;

impl BannedWordMapper {
    pub fn from_model(model: BannedWordModel) -> BannedWord {
        BannedWord {
            id: model.id,
            word: model.word,
            active: model.active,
        }
    }
}