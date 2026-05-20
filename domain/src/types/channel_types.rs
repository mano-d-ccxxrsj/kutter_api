pub struct CreateChannelCommand {
    pub user_id: i32,
    pub community_id: i32,
    pub name: String,
    pub topic: Option<String>,
    pub hidden: bool,
}