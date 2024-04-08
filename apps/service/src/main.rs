mod config;
mod domain;
mod infrastructure;
mod presentation;
mod usecase;
use crate::presentation::presentation::BlogServer;
pub mod team_blog {
    tonic::include_proto!("blog");
}

// sea-orm-cli migrate refresh
// sea-orm-cli generate entity -o src/infrastructure/entities

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50052".parse()?;
    let service = BlogServer::new();
    service.run_server(addr).await?;
    Ok(())
}
