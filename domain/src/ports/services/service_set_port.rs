use crate::ports::services::auth_service_port::AuthServicePort;
use crate::ports::services::channel_service_port::ChannelServicePort;
use crate::ports::services::community_service_port::CommunityServicePort;
use crate::ports::services::message_service_port::MessageServicePort;

pub trait ServiceSetPort: Send + Sync {
    fn auth_service(&self) -> &dyn AuthServicePort;
    fn channel_service(&self) -> &dyn ChannelServicePort;
    fn community_service(&self) -> &dyn CommunityServicePort;
    fn message_service(&self) -> &dyn MessageServicePort;
}