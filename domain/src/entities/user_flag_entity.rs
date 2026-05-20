use chrono::{DateTime, Utc};

pub struct UserFlag {
    pub id: i32,
    pub user_id: i32,
    pub field: String,
    pub action: String,
    pub target: String,
    pub attempted_text: String,
    pub matched_words: String,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub struct NewUserFlag {
    pub user_id: i32,
    pub field: String,
    pub action: String,
    pub target: String,
    pub attempted_text: String,
    pub matched_words: String,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
}