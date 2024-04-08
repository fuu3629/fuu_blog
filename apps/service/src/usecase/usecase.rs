use crate::domain::auth::AuthDomain;
use crate::team_blog::{Blog, Blogs, CreateUserRequest, LoginRequest, PostBlog, Token};
use crate::{infrastructure::infrastructure::InfrastructureImpl, team_blog::Member};
use bcrypt::verify;
use tonic::{Request, Status};

#[derive(Default)]
pub struct UsecaseImpl {
    infrastructure: InfrastructureImpl,
    auth_domain: AuthDomain,
}

impl UsecaseImpl {
    pub fn new() -> Self {
        UsecaseImpl {
            infrastructure: InfrastructureImpl::default(),
            auth_domain: AuthDomain::default(),
        }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<String, Status> {
        let qiita_id = request.qiita_id.clone();
        let mut _token = String::new();
        let hash_password = bcrypt::hash(request.password, 10).unwrap();
        match qiita_id {
            Some(qiita_id) => {
                println!("{:?}", qiita_id);
                let is_user = self
                    .infrastructure
                    .find_user_by_name(qiita_id.clone())
                    .await?;
                println!("{:?}", is_user);
                if !is_user {
                    return Err(Status::invalid_argument("qiita_id is not found"));
                }
                let user_id = self
                    .infrastructure
                    .create_user(
                        request.name,
                        hash_password,
                        request.qiita_api_key,
                        request.qiita_id,
                    )
                    .await?;
                println!("{:?}", user_id);
                let blogs = self.infrastructure.get_blog_by_user(vec![qiita_id]).await?;
                println!("{:?}", blogs);
                self.infrastructure.register_blog(user_id, blogs).await?;
                _token = self.auth_domain.generate_token(user_id)?;
            }
            None => {
                let user_id = self
                    .infrastructure
                    .create_user(
                        request.name,
                        hash_password,
                        request.qiita_api_key,
                        request.qiita_id,
                    )
                    .await?;
                _token = self.auth_domain.generate_token(user_id)?;
            }
        };
        Ok(_token)
    }

    pub async fn delete_user(&self, request: Request<()>) -> Result<(), Status> {
        let user_id = self.auth_domain.auth(request)?;
        self.infrastructure.delete_user(user_id).await?;
        self.infrastructure.delete_blog_by_user_id(user_id).await?;
        Ok(())
    }

    pub async fn login(&self, request: LoginRequest) -> Result<Token, Status> {
        let user = self.infrastructure.get_user_by_name(request.name).await?;
        match verify(request.password, &user.password) {
            Ok(true) => {
                let token = self.auth_domain.generate_token(user.id)?;
                Ok(Token { token })
            }
            Ok(false) | Err(_) => {
                return Err(Status::failed_precondition("Invalid password"));
            }
        }
    }

    pub async fn get_members(&self) -> Result<Vec<Member>, Status> {
        let user = self.infrastructure.get_members().await?;
        let members: Vec<Member> = user.into_iter().map(|user| Member::from(user)).collect();
        Ok(members)
    }

    pub async fn get_blog_by_user(&self, ids: Vec<String>) -> Result<Blogs, Status> {
        let res = self.infrastructure.get_blog_by_user(ids).await?;
        let blogs: Blogs = Blogs {
            blogs: res.into_iter().map(|item| Blog::from(item)).collect(),
        };
        Ok(blogs)
    }

    pub async fn get_blog_by_id(&self, id: String) -> Result<Blog, Status> {
        let res = self.infrastructure.get_blog_by_id(id).await?;
        Ok(Blog::from(res))
    }

    pub async fn post_blog(&self, request: Request<PostBlog>) -> Result<(), Status> {
        let user_id = self.auth_domain.auth(request)?;
        let user = self.infrastructure.get_user_by_id(user_id).await?;
        Ok(())
    }
}
