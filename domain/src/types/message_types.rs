pub struct SendChannelMessageCommand {
    pub user_id: i32,
    pub community_id: i32,
    pub channel_id: i32,
    pub message: String,
    pub replied_message: Option<i32>,
}

pub struct ListChannelMessagesCommand {
    pub user_id: i32,
    pub community_id: i32,
    pub channel_id: i32,
}

pub struct EditChannelMessageCommand {
    pub user_id: i32,
    pub message_id: i32,
    pub message: String,
}

pub struct DeleteChannelMessageCommand {
    pub user_id: i32,
    pub message_id: i32,
}