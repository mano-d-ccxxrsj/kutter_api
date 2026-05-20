pub struct Member {
    pub id: i32,
    pub user_id: i32,
    pub community_id: i32,
    pub owner: bool,
    pub admin: bool,
}

pub struct NewMember {
    pub user_id: i32,
    pub community_id: i32,
    pub owner: bool,
    pub admin: bool,
}