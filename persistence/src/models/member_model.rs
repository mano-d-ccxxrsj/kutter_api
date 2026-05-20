#[derive(sqlx::FromRow)]
pub struct MemberModel {
    pub id: i32,
    pub user_id: i32,
    pub community_id: i32,
    pub owner: bool,
    pub admin: bool,
}