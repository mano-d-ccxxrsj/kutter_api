use crate::chat::types::ChatMessageService;
use persistence::entity::models::ChatMessageRepository;

impl ChatMessageService {
    pub fn new(repository: ChatMessageRepository) -> ChatMessageService {
        ChatMessageService { repository }
    }
}