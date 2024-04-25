use crate::{
    domain::qiita::QiitaItem,
    infrastructure::entities::{post, tag},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Post {
    pub id: i64,
    pub user_id: String,
    pub created_at: String,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}

impl From<QiitaItem> for Post {
    fn from(item: QiitaItem) -> Self {
        Post {
            //QiitaItemからの変換以降はロジックで対応
            id: 0,
            user_id: "".to_string(),
            created_at: item.created_at,
            title: item.title,
            body: item.body,
            tags: item.tags.iter().map(|tag| tag.name.clone()).collect(),
        }
    }
}

impl From<(post::Model, Vec<tag::Model>)> for Post {
    fn from(item: (post::Model, Vec<tag::Model>)) -> Self {
        let (post, tags) = item;
        Post {
            id: post.id,
            user_id: post.user_id,
            created_at: post.created_at,
            title: post.title,
            body: post.body,
            tags: tags.iter().map(|tag| tag.name.clone()).collect(),
        }
    }
}
