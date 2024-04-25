use super::error::InfrastructureError;
use super::qiita_client::QiitaClient;
use crate::config;
use crate::domain::entity::post::Post;
use crate::domain::prompts::Prompts;
use crate::domain::qiita::QiitaItem;
use chatgpt::prelude::{ChatGPT, ModelConfigurationBuilder};
use chatgpt::types::ResponseChunk;
use futures_core::stream::Stream;
use sea_orm::QuerySelect;
use std::time::Duration;

#[derive(Default)]
pub struct InfrastructureImpl {}

impl InfrastructureImpl {
    //ユーザーがいるかどうかの確認
    pub async fn find_user_by_name(&self, qiita_id: &String) -> Result<bool, InfrastructureError> {
        let client = QiitaClient::new(format!("Bearer {}", &config::CONFIG.qiita_api_key).as_str());
        let is_user_exist = client.find_user_by_name(qiita_id.as_str()).await?;
        Ok(is_user_exist)
    }

    pub async fn fetch_post_by_user(
        &self,
        ids: Vec<String>,
    ) -> Result<Vec<QiitaItem>, InfrastructureError> {
        let client = QiitaClient::new(format!("Bearer {}", &config::CONFIG.qiita_api_key).as_str());
        let query = format!("page=1&per_page=100&query=user:{}", ids.join(","));
        println!("{:?}", query);
        let res = client
            .get::<Vec<QiitaItem>>(&format!("https://qiita.com/api/v2/items?{}", query))
            .await?;
        Ok(res)
    }

    pub async fn fetch_summary(&self, post: Post) -> Result<String, InfrastructureError> {
        let client = ChatGPT::new_with_config(
            &config::CONFIG.gpt_api_key,
            ModelConfigurationBuilder::default()
                .timeout(Duration::from_secs(100))
                .build()
                .unwrap(),
        )?;
        let prompts = Prompts::new(post.body);
        let res = client.send_message(prompts.summary_pre_prompt).await?;
        println!("{:?}", res);
        Ok(res.message().content.clone())
    }

    pub async fn fetch_summary_stream(
        &self,
        post: Post,
    ) -> Result<impl Stream<Item = ResponseChunk>, InfrastructureError> {
        let client = ChatGPT::new_with_config(
            &config::CONFIG.gpt_api_key,
            ModelConfigurationBuilder::default()
                .timeout(Duration::from_secs(100))
                .build()
                .unwrap(),
        )?;
        let prompts = Prompts::new(post.body);
        let stream = client
            .send_message_streaming(prompts.summary_pre_prompt)
            .await?;
        Ok(stream)
    }
}
