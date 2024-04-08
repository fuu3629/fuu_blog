use crate::team_blog::blog_service_server::BlogServiceServer;
use crate::team_blog::{
    Blog, Blogs, CreateUserRequest, GetBlogByIdRequest, GetBlogByUserRequest, GetMembersResponse,
    LoginRequest, PostBlog, Token,
};
use crate::{team_blog::blog_service_server, usecase::usecase::UsecaseImpl};
use std::net::SocketAddr;
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
    ) -> Result<Response<Token>, Status> {
        let token = self.usecase.create_user(request.into_inner()).await?;
        Ok(Response::new(Token { token }))
    }

    async fn delete_user(&self, request: Request<()>) -> Result<Response<()>, Status> {
        self.usecase.delete_user(request).await?;
        Ok(Response::new(()))
    }

    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<Token>, Status> {
        let token = self.usecase.login(request.into_inner()).await?;
        Ok(Response::new(token))
    }

    async fn get_members(
        &self,
        _request: Request<()>,
    ) -> Result<Response<GetMembersResponse>, Status> {
        let members = self.usecase.get_members().await?;
        Ok(Response::new(GetMembersResponse { members }))
    }

    async fn get_blog_by_user(
        &self,
        request: Request<GetBlogByUserRequest>,
    ) -> Result<Response<Blogs>, Status> {
        let req = request.into_inner();
        let blogs = self.usecase.get_blog_by_user(req.ids).await?;
        Ok(Response::new(blogs))
    }

    async fn get_blog_by_id(
        &self,
        request: Request<GetBlogByIdRequest>,
    ) -> Result<Response<Blog>, Status> {
        let req = request.into_inner();
        let blogs = self.usecase.get_blog_by_id(req.id).await?;
        Ok(Response::new(blogs))
    }

    async fn post_blog(&self, request: Request<PostBlog>) -> Result<Response<()>, Status> {
        self.usecase.post_blog(request).await?;
        Ok(Response::new(()))
    }
}
