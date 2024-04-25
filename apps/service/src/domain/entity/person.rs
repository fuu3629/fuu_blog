#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub id: i64,
    pub user_id: String,
    pub name: String,
    pub qiita_id: Option<String>,
    pub avatar: Option<Vec<u8>>,
    pub password: String,
    pub qiita_api_key: Option<String>,
}
