use crate::friend::types::FriendService;
use persistence::entity::models::FriendRepository;

impl FriendService {
    pub fn new(repository: FriendRepository) -> FriendService {
        FriendService { repository }
    }
}