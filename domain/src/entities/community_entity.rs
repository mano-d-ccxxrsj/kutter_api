pub struct Community {
    pub id: i32,
    pub name: String,
    pub about: Option<String>,
    pub nsfw: bool,
}

pub struct NewCommunity {
    pub name: String,
    pub about: Option<String>,
    pub nsfw: bool,
}