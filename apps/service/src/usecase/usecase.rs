use crate::domain::auth::AuthDomain;
use crate::domain::entity::post::Post;
use crate::infrastructure::repository::{
    person_repository::PersonRepository, post_repository::PostRepository,
};
use crate::team_blog::{CreateUserRequest, GetSummaryResponse, LoginRequest, Pagination, PostBlog};
use crate::{infrastructure::infrastructure::InfrastructureImpl, team_blog::Member};
use bcrypt::verify;
use chatgpt::types::ResponseChunk;
use futures_core::stream::Stream;
use tonic::{Request, Status};

#[derive(Default)]
pub struct UsecaseImpl {
    infrastructure: InfrastructureImpl,
    post_repository: PostRepository,
    person_repository: PersonRepository,
    auth_domain: AuthDomain,
}

impl UsecaseImpl {
    pub fn new() -> Self {
        UsecaseImpl {
            infrastructure: InfrastructureImpl::default(),
            post_repository: PostRepository::default(),
            person_repository: PersonRepository::default(),
            auth_domain: AuthDomain::default(),
        }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<String, Status> {
        let qiita_id = request.qiita_id.clone();
        let hash_password = bcrypt::hash(request.password, 10).unwrap();
        let token = match qiita_id {
            Some(qiita_id) => {
                let is_user = self.infrastructure.find_user_by_name(&qiita_id).await?;
                if !is_user {
                    return Err(Status::invalid_argument("qiita_id is not found"));
                }
                self.person_repository
                    .save(
                        &request.user_id,
                        &request.name,
                        &hash_password,
                        &request.qiita_api_key,
                        &request.qiita_id,
                    )
                    .await?;
                let qiita_blogs = self
                    .infrastructure
                    .fetch_post_by_user(vec![qiita_id])
                    .await?;
                let posts: Vec<Post> = qiita_blogs
                    .into_iter()
                    .map(|item| Post::from(item))
                    .collect();
                for post in posts {
                    self.post_repository.save(&request.user_id, post).await?;
                }
                self.auth_domain.generate_token(&request.user_id)?
            }
            None => {
                self.person_repository
                    .save(
                        &request.user_id,
                        &request.name,
                        &hash_password,
                        &request.qiita_api_key,
                        &request.qiita_id,
                    )
                    .await?;
                self.auth_domain.generate_token(&request.user_id)?
            }
        };
        Ok(token)
    }

    pub async fn delete_user(&self, request: Request<()>) -> Result<(), Status> {
        let user_id = self.auth_domain.auth(request)?;
        self.person_repository.delete(&user_id).await?;
        Ok(())
    }

    pub async fn login(&self, request: LoginRequest) -> Result<String, Status> {
        let user = self.person_repository.find_by_id(&request.user_id).await?;
        match verify(request.password, &user.password) {
            Ok(true) => {
                let token = self.auth_domain.generate_token(&user.user_id)?;
                Ok(token)
            }
            Ok(false) | Err(_) => {
                return Err(Status::failed_precondition("Invalid password"));
            }
        }
    }

    pub async fn get_members(&self) -> Result<Vec<Member>, Status> {
        let user = self.person_repository.find_all().await?;
        let members: Vec<Member> = user.into_iter().map(|user| Member::from(user)).collect();
        Ok(members)
    }

    pub async fn get_blog_by_user(
        &self,
        ids: Vec<String>,
        pagenation: Pagination,
    ) -> Result<(Vec<Post>, i32), Status> {
        let (res, total_count) = self
            .post_repository
            .find_by_user_ids(
                ids,
                &pagenation.page,
                &pagenation.page_size,
                &pagenation.order,
            )
            .await?;
        let blogs = res
            .into_iter()
            .map(|(blog, tags)| Post::from((blog, tags)))
            .collect();
        Ok((blogs, total_count as i32))
    }

    pub async fn get_blog_by_id(&self, id: i64) -> Result<Post, Status> {
        let blog = self.post_repository.find_by_id(&id).await?;
        Ok(Post::from(blog))
    }

    //TODO post機能を実装する
    pub async fn post_blog(&self, request: Request<PostBlog>) -> Result<(), Status> {
        let user_id = self.auth_domain.auth(request)?;
        let _user = self.person_repository.find_by_id(&user_id).await?;
        Ok(())
    }

    pub async fn get_summary(&self, blog_id: i64) -> Result<GetSummaryResponse, Status> {
        let blog = self.post_repository.find_by_id(&blog_id).await?;
        let summary = self.infrastructure.fetch_summary(Post::from(blog)).await?;
        Ok(GetSummaryResponse {
            summary_text: summary,
        })
    }

    pub async fn get_summary_stream(
        &self,
        blog_id: i64,
    ) -> Result<impl Stream<Item = ResponseChunk>, Status> {
        let blog = self.post_repository.find_by_id(&blog_id).await?;
        let summary_stream = self
            .infrastructure
            .fetch_summary_stream(Post::from(blog))
            .await?;
        Ok(summary_stream)
    }
}
