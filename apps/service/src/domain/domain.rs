use crate::infrastructure::entities::post::Model as PostModel;
use crate::infrastructure::entities::user::Model as UserModel;
use crate::team_blog::Blog;
use crate::team_blog::BlogPreview;
use crate::team_blog::Member;

impl From<UserModel> for Member {
    fn from(user: UserModel) -> Self {
        Member {
            user_id: user.user_id,
            name: user.name,
        }
    }
}

// impl From<QiitaTag> for Tag {
//     fn from(tag: QiitaTag) -> Self {
//         Tag { name: tag.name }
//     }
// }

//TODO Blogのエンティティにurlを追加する
impl From<PostModel> for Blog {
    fn from(item: PostModel) -> Self {
        Blog {
            id: item.id,
            title: item.title,
            created_at: item.created_at,
            body: item.body,
            tags: vec![],
            user_id: item.user_id,
        }
    }
}

impl From<PostModel> for BlogPreview {
    fn from(item: PostModel) -> Self {
        BlogPreview {
            id: item.id,
            title: item.title,
            created_at: item.created_at,
            tags: vec![],
            user_id: item.user_id,
        }
    }
}

// impl From<TagModel> for Tag {
//     fn from(tag: TagModel) -> Self {
//         Tag { name: tag.name }
//     }
// }
