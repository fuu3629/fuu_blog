use crate::config;
use crate::domain::prompts::Prompts;
use crate::domain::qiita::QiitaItem;
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};

use super::entities::user::Entity as User;
use super::entities::{blog, tag, user};
use super::error::InfrastructureError;
use super::lib::establish_connection;
use super::qiita_client::QiitaClient;
use chatgpt::prelude::{ChatGPT, ModelConfigurationBuilder};
use std::time::Duration;

#[derive(Default)]
pub struct InfrastructureImpl {}

impl InfrastructureImpl {
    pub async fn create_user(
        &self,
        user_id: &String,
        name: &String,
        hash_password: &String,
        qiita_api_key: &Option<String>,
        qiita_id: &Option<String>,
    ) -> Result<i64, InfrastructureError> {
        let db = establish_connection().await?;
        let user = user::ActiveModel {
            user_id: ActiveValue::set(user_id.to_string()),
            name: ActiveValue::set(name.to_string()),
            password: ActiveValue::set(hash_password.to_string()),
            qiita_api_key: ActiveValue::set(qiita_api_key.clone()),
            qiita_id: ActiveValue::set(qiita_id.clone()),
            avatar: ActiveValue::set(None),
            ..Default::default()
        };
        let res = User::insert(user).exec(&db).await?;
        let user_id = res.last_insert_id;
        Ok(user_id)
    }

    pub async fn delete_user(&self, user_id: i64) -> Result<(), InfrastructureError> {
        let db = establish_connection().await?;
        User::delete_by_id(user_id).exec(&db).await?;
        Ok(())
    }

    pub async fn delete_blog_by_user_id(&self, user_id: i64) -> Result<(), InfrastructureError> {
        let db = establish_connection().await?;
        blog::Entity::delete_many()
            .filter(blog::Column::UserId.eq(user_id))
            .exec(&db)
            .await?;
        tag::Entity::delete_many()
            .filter(tag::Column::UserId.eq(user_id))
            .exec(&db)
            .await?;
        Ok(())
    }

    pub async fn get_members(&self) -> Result<Vec<user::Model>, InfrastructureError> {
        let db = establish_connection().await?;
        let members: Vec<user::Model> = User::find().all(&db).await?;
        Ok(members)
    }

    pub async fn get_user_by_user_id(
        &self,
        user_id: String,
    ) -> Result<user::Model, InfrastructureError> {
        let db = establish_connection().await?;
        let user = User::find()
            .filter(user::Column::UserId.eq(user_id))
            .one(&db)
            .await?
            .unwrap();
        Ok(user)
    }

    pub async fn get_blog_by_id(&self, id: i64) -> Result<blog::Model, InfrastructureError> {
        let db = establish_connection().await?;
        let blog = blog::Entity::find_by_id(id).one(&db).await?.unwrap();
        Ok(blog)
    }

    pub async fn get_blog_by_user_ids(
        &self,
        ids: Vec<String>,
        page: &i32,
        page_size: &i32,
        order: &i32,
    ) -> Result<(Vec<blog::Model>, u64), InfrastructureError> {
        let db = establish_connection().await?;
        let total_count = blog::Entity::find()
            .filter(blog::Column::UserId.is_in(&ids))
            .count(&db)
            .await?;
        let slect_blog = blog::Entity::find().filter(blog::Column::UserId.is_in(ids));
        let order_blogs = match order {
            0 => slect_blog.order_by_asc(blog::Column::CreatedAt),
            1 => slect_blog.order_by_desc(blog::Column::CreatedAt),
            _ => slect_blog.order_by_asc(blog::Column::CreatedAt),
        };
        let offset = ((page - 1) * page_size) as u64;
        let limit = page_size.clone() as u64;
        let blogs = order_blogs.offset(offset).limit(limit).all(&db).await?;
        Ok((blogs, total_count))
    }

    pub async fn get_tags_by_blog_id(
        &self,
        blog_id: i64,
    ) -> Result<Vec<tag::Model>, InfrastructureError> {
        let db = establish_connection().await?;
        let tags = tag::Entity::find()
            .filter(tag::Column::BlogId.eq(blog_id))
            .all(&db)
            .await?;
        Ok(tags)
    }

    pub async fn get_user_by_id(&self, user_id: i64) -> Result<user::Model, InfrastructureError> {
        let db = establish_connection().await?;
        let user = User::find_by_id(user_id).one(&db).await?.unwrap();
        Ok(user)
    }

    pub async fn register_blog(
        &self,
        user_id: &String,
        blogs: Vec<QiitaItem>,
    ) -> Result<(), InfrastructureError> {
        let db = establish_connection().await?;
        let blog = blogs
            .clone()
            .iter()
            .map(|item| blog::ActiveModel {
                user_id: ActiveValue::set(user_id.to_string()),
                title: ActiveValue::set(item.title.clone()),
                created_at: ActiveValue::set(item.created_at.clone()),
                body: ActiveValue::set(item.body.clone()),
                ..Default::default()
            })
            .collect::<Vec<blog::ActiveModel>>();
        blog::Entity::insert_many(blog).exec(&db).await?;
        let blog_by_user_id = blog::Entity::find()
            .filter(blog::Column::UserId.eq(user_id))
            .all(&db)
            .await?;
        let mut tags: Vec<tag::ActiveModel> = vec![];
        for item in blogs {
            let blog_id = blog_by_user_id
                .iter()
                .find(|blog| blog.title == item.title)
                .unwrap()
                .id;
            let tag = item
                .tags
                .clone()
                .iter()
                .map(|tag| tag::ActiveModel {
                    blog_id: ActiveValue::set(blog_id),
                    name: ActiveValue::set(tag.clone().name),
                    user_id: ActiveValue::set(user_id.to_string()),
                    ..Default::default()
                })
                .collect::<Vec<tag::ActiveModel>>();
            tags.extend(tag);
        }
        tag::Entity::insert_many(tags).exec(&db).await?;
        Ok(())
    }

    //ユーザーがいるかどうかの確認
    pub async fn find_user_by_name(&self, qiita_id: &String) -> Result<bool, InfrastructureError> {
        let client = QiitaClient::new(format!("Bearer {}", &config::CONFIG.qiita_api_key).as_str());
        let is_user_exist = client.find_user_by_name(qiita_id.as_str()).await?;
        Ok(is_user_exist)
    }

    pub async fn fetch_blog_by_user(
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

    pub async fn fetch_summary(&self, blog: blog::Model) -> Result<String, InfrastructureError> {
        let client = ChatGPT::new_with_config(
            &config::CONFIG.gpt_api_key,
            ModelConfigurationBuilder::default()
                .timeout(Duration::from_secs(100))
                .build()
                .unwrap(),
        )?;
        let prompts = Prompts::new(blog.body);
        let res = client.send_message(prompts.summary_pre_prompt).await?;
        println!("{:?}", res);
        Ok(res.message().content.clone())
    }
}
