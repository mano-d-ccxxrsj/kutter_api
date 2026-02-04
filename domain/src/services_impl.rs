use crate::services::ServiceSet;
use crate::user::types::UserService;
use crate::chat::types::ChatMessageService;
use crate::friend::types::FriendService;
use persistence::database::types::RepositorySet;
use shared::services::ports::ServicesPort;

impl ServicesPort for ServiceSet {}

impl ServiceSet {
    pub fn new(repos: RepositorySet) -> ServiceSet {
        ServiceSet {
            user: UserService::new(repos.user_repo),
            chat: ChatMessageService::new(repos.chat_message_repo),
            friend: FriendService::new(repos.friend_repo),
        }
    }
}