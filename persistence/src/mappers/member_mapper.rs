use domain::entities::member_entity::Member;

use crate::models::member_model::MemberModel;

pub struct MemberMapper;

impl MemberMapper {
    pub fn from_model(model: MemberModel) -> Member {
        Member {
            id: model.id,
            user_id: model.user_id,
            community_id: model.community_id,
            owner: model.owner,
            admin: model.admin,
        }
    }
}