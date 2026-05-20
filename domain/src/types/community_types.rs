pub struct CreateCommunityCommand {
    pub owner_user_id: i32,
    pub name: String,
    pub about: Option<String>,
    pub nsfw: bool,
}