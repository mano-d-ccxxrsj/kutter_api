use crate::user::types::UserService;
use crate::chat::types::ChatMessageService;
use crate::friend::types::FriendService;

pub struct ServiceSet {
    pub user: UserService,
    pub chat: ChatMessageService,
    pub friend: FriendService,
}