use super::qiita::QiitaTag;
use crate::infrastructure::entities::blog::Model as BlogModel;
use crate::infrastructure::entities::tag::Model as TagModel;
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

//TODO Blogのエンティティにurlを追加する
impl From<BlogModel> for Blog {
    fn from(item: BlogModel) -> Self {
        Blog {
            id: item.id,
            title: item.title,
            url: "".to_string(),
            created_at: item.created_at,
            body: item.body,
            tags: vec![],
        }
    }
}

impl From<TagModel> for Tag {
    fn from(tag: TagModel) -> Self {
        Tag { name: tag.name }
    }
}
