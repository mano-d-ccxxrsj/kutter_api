pub struct Channel {
    pub id: i32,
    pub community_id: i32,
    pub name: String,
    pub topic: Option<String>,
    pub hidden: bool,
}

pub struct NewChannel {
    pub community_id: i32,
    pub name: String,
    pub topic: Option<String>,
    pub hidden: bool,
}