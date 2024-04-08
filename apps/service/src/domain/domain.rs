use super::qiita::{QiitaItem, QiitaTag};
use crate::infrastructure::entities::user::Model as UserModel;
use crate::team_blog::Blog;
use crate::team_blog::Member;
use crate::team_blog::Tag;

impl From<UserModel> for Member {
    fn from(user: UserModel) -> Self {
        Member {
            id: user.id as i64,
            name: user.name,
        }
    }
}

impl From<QiitaTag> for Tag {
    fn from(tag: QiitaTag) -> Self {
        Tag { name: tag.name }
    }
}

impl From<QiitaItem> for Blog {
    fn from(item: QiitaItem) -> Self {
        Blog {
            id: item.id,
            title: item.title,
            url: item.url,
            created_at: item.created_at,
            body: item.body,
            tags: item.tags.into_iter().map(|tag| tag.into()).collect(),
        }
    }
}
