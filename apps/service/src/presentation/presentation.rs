use crate::team_blog::blog_service_server::BlogServiceServer;
use crate::team_blog::{
    CreateUserRequest, CreateUserResponse, GetBlogByIdRequest, GetBlogByIdResponse,
    GetBlogByUsersRequest, GetBlogByUsersResponse, GetMembersResponse, GetSummaryRequest,
    GetSummaryResponse, LoginRequest, LoginResponse, PostBlog,
};
use crate::{team_blog::blog_service_server, usecase::usecase::UsecaseImpl};
use chatgpt::types::ResponseChunk;
use futures::StreamExt;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder;
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;

#[derive(Default)]
pub struct BlogServer {
    usecase: UsecaseImpl,
}

impl BlogServer {
    pub fn new() -> Self {
        BlogServer {
            usecase: UsecaseImpl::new(),
        }
    }

    pub async fn run_server(self, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let allow_cors = CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any)
            .allow_methods(tower_http::cors::Any);
        Server::builder()
            .accept_http1(true) // gRPC-webに対応するために必要
            .layer(allow_cors) // CORSに対応するために必要
            .layer(GrpcWebLayer::new()) // gRPC-webに対応するために必要
            .add_service(BlogServiceServer::new(self))
            .add_service(
                Builder::configure()
                    .register_encoded_file_descriptor_set(tonic::include_file_descriptor_set!(
                        "store_descriptor"
                    ))
                    .build()
                    .unwrap(),
            )
            .serve(addr)
            .await?;
        Ok(())
    }
}

#[tonic::async_trait]
impl blog_service_server::BlogService for BlogServer {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let token = self.usecase.create_user(request.into_inner()).await?;
        Ok(Response::new(CreateUserResponse::from(token)))
    }

    async fn delete_user(&self, request: Request<()>) -> Result<Response<()>, Status> {
        self.usecase.delete_user(request).await?;
        Ok(Response::new(()))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let token = self.usecase.login(request.into_inner()).await?;
        Ok(Response::new(LoginResponse::from(token)))
    }

    async fn get_members(
        &self,
        _request: Request<()>,
    ) -> Result<Response<GetMembersResponse>, Status> {
        let members = self.usecase.get_members().await?;
        Ok(Response::new(GetMembersResponse { members }))
    }

    async fn get_blog_by_users(
        &self,
        request: Request<GetBlogByUsersRequest>,
    ) -> Result<Response<GetBlogByUsersResponse>, Status> {
        let req = request.into_inner();
        let (blogs, total_count) = self
            .usecase
            .get_blog_by_user(req.ids, req.pagination.unwrap())
            .await?;
        Ok(Response::new(GetBlogByUsersResponse::from((
            blogs,
            total_count,
        ))))
    }

    async fn get_blog_by_id(
        &self,
        request: Request<GetBlogByIdRequest>,
    ) -> Result<Response<GetBlogByIdResponse>, Status> {
        let req = request.into_inner();
        let blog = self.usecase.get_blog_by_id(req.id).await?;
        Ok(Response::new(GetBlogByIdResponse::from(blog)))
    }

    async fn post_blog(&self, request: Request<PostBlog>) -> Result<Response<()>, Status> {
        self.usecase.post_blog(request).await?;
        Ok(Response::new(()))
    }
    async fn get_summary(
        &self,
        request: Request<GetSummaryRequest>,
    ) -> Result<Response<GetSummaryResponse>, Status> {
        let req = request.into_inner();
        let summary = self.usecase.get_summary(req.blog_id).await?;
        Ok(Response::new(summary))
    }

    type getSummaryStreamStream = ReceiverStream<Result<GetSummaryResponse, Status>>;

    async fn get_summary_stream(
        &self,
        request: Request<GetSummaryRequest>,
    ) -> Result<Response<Self::getSummaryStreamStream>, Status> {
        let req = request.into_inner();
        let summary = self.usecase.get_summary_stream(req.blog_id).await?;
        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            summary
                .for_each(|each| async {
                    match each {
                        ResponseChunk::Content {
                            delta,
                            response_index: _,
                        } => tx
                            .send(Ok(GetSummaryResponse {
                                summary_text: delta,
                            }))
                            .await
                            .unwrap(),
                        _ => tx
                            .send(Ok(GetSummaryResponse {
                                summary_text: "".to_string(),
                            }))
                            .await
                            .unwrap(),
                    }
                })
                .await;
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
