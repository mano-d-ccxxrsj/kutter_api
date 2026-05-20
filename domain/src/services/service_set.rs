use crate::ports::services::auth_service_port::AuthServicePort;
use crate::ports::services::channel_service_port::ChannelServicePort;
use crate::ports::services::community_service_port::CommunityServicePort;
use crate::ports::services::message_service_port::MessageServicePort;
use crate::ports::services::service_set_port::ServiceSetPort;
use crate::services::types::WebServiceSet;

impl WebServiceSet {
    pub fn new(
        auth_service: Box<dyn AuthServicePort>,
        channel_service: Box<dyn ChannelServicePort>,
        community_service: Box<dyn CommunityServicePort>,
        message_service: Box<dyn MessageServicePort>,
    ) -> Self {
        Self { auth_service, channel_service, community_service, message_service }
    }
}

impl ServiceSetPort for WebServiceSet {
    fn auth_service(&self) -> &dyn AuthServicePort {
        self.auth_service.as_ref()
    }

    fn channel_service(&self) -> &dyn ChannelServicePort {
        self.channel_service.as_ref()
    }

    fn community_service(&self) -> &dyn CommunityServicePort {
        self.community_service.as_ref()
    }

    fn message_service(&self) -> &dyn MessageServicePort {
        self.message_service.as_ref()
    }
}