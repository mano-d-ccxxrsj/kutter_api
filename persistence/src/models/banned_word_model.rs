#[derive(sqlx::FromRow)]
pub struct BannedWordModel {
    pub id: i32,
    pub word: String,
    pub active: bool,
}