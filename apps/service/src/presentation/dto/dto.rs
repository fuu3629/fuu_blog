use crate::domain::entity::post::Post;
use crate::infrastructure::entities::user::Model as UserModel;
use crate::team_blog::{
    Blog, BlogPreview, CreateUserResponse, GetBlogByIdResponse, GetBlogByUsersResponse,
    GetMembersResponse, LoginResponse,
};

impl From<String> for CreateUserResponse {
    fn from(token: String) -> Self {
        CreateUserResponse { token }
    }
}

impl From<String> for LoginResponse {
    fn from(token: String) -> Self {
        LoginResponse { token }
    }
}

impl From<Vec<UserModel>> for GetMembersResponse {
    fn from(members: Vec<UserModel>) -> Self {
        GetMembersResponse {
            members: members.into_iter().map(|m| m.into()).collect(),
        }
    }
}

impl From<Post> for Blog {
    fn from(blog: Post) -> Self {
        Blog {
            id: blog.id,
            user_id: blog.user_id,
            created_at: blog.created_at,
            title: blog.title,
            tags: blog.tags,
            body: blog.body,
        }
    }
}

impl From<Post> for GetBlogByIdResponse {
    fn from(blog: Post) -> Self {
        GetBlogByIdResponse {
            id: blog.id,
            user_id: blog.user_id,
            created_at: blog.created_at,
            title: blog.title,
            tags: blog.tags,
            body: blog.body,
        }
    }
}

impl From<Post> for BlogPreview {
    fn from(blog: Post) -> Self {
        BlogPreview {
            id: blog.id,
            user_id: blog.user_id,
            created_at: blog.created_at,
            title: blog.title,
            tags: blog.tags,
        }
    }
}

impl From<(Vec<Post>, i32)> for GetBlogByUsersResponse {
    fn from((blogs, total_count): (Vec<Post>, i32)) -> Self {
        GetBlogByUsersResponse {
            blogs: blogs.into_iter().map(|b| b.into()).collect(),
            total_count,
        }
    }
}
