use crate::user::types::UserService;
use persistence::entity::models::UserRepository;

impl UserService {
    pub fn new(repository: UserRepository) -> UserService {
        UserService { repository }
    }
}