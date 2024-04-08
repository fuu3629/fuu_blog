use crate::config;
use crate::domain::qiita::QiitaItem;
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

use super::entities::user::Entity as User;
use super::entities::{blog, tag, user};
use super::error::InfrastructureError;
use super::lib::establish_connection;
use super::qiita_client::QiitaClient;

#[derive(Default)]
pub struct InfrastructureImpl {}

impl InfrastructureImpl {
    pub async fn create_user(
        &self,
        name: String,
        hash_password: String,
        qiita_api_key: Option<String>,
        qiita_id: Option<String>,
    ) -> Result<i64, InfrastructureError> {
        let db = establish_connection().await?;
        let user = user::ActiveModel {
            name: ActiveValue::set(name),
            password: ActiveValue::set(hash_password),
            qiita_api_key: ActiveValue::set(qiita_api_key),
            qiita_id: ActiveValue::set(qiita_id),
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

    pub async fn get_user_by_name(
        &self,
        user_name: String,
    ) -> Result<user::Model, InfrastructureError> {
        let db = establish_connection().await?;
        let user = User::find()
            .filter(user::Column::Name.eq(user_name))
            .one(&db)
            .await?
            .unwrap();
        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: i64) -> Result<user::Model, InfrastructureError> {
        let db = establish_connection().await?;
        let user = User::find_by_id(user_id).one(&db).await?.unwrap();
        Ok(user)
    }

    pub async fn register_blog(
        &self,
        user_id: i64,
        blogs: Vec<QiitaItem>,
    ) -> Result<(), InfrastructureError> {
        let db = establish_connection().await?;
        let blog = blogs
            .clone()
            .iter()
            .map(|item| blog::ActiveModel {
                user_id: ActiveValue::set(user_id),
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
                    user_id: ActiveValue::set(user_id),
                    ..Default::default()
                })
                .collect::<Vec<tag::ActiveModel>>();
            tags.extend(tag);
        }
        tag::Entity::insert_many(tags).exec(&db).await?;
        Ok(())
    }

    //ユーザーがいるかどうかの確認
    pub async fn find_user_by_name(&self, qiita_id: String) -> Result<bool, InfrastructureError> {
        let client = QiitaClient::new(format!("Bearer {}", config::CONFIG.qiita_api_key).as_str());
        let is_user_exist = client.find_user_by_name(qiita_id.as_str()).await?;
        Ok(is_user_exist)
    }

    pub async fn get_blog_by_user(
        &self,
        ids: Vec<String>,
    ) -> Result<Vec<QiitaItem>, InfrastructureError> {
        let client = QiitaClient::new(format!("Bearer {}", config::CONFIG.qiita_api_key).as_str());
        let query = format!("page=1&per_page=10&query=user:{}", ids.join(","));
        let res = client
            .get::<Vec<QiitaItem>>(&format!("https://qiita.com/api/v2/items?{}", query))
            .await?;
        Ok(res)
    }

    pub async fn get_blog_by_id(&self, id: String) -> Result<QiitaItem, InfrastructureError> {
        let client = QiitaClient::new(format!("Bearer {}", config::CONFIG.qiita_api_key).as_str());
        let res = client
            .get::<QiitaItem>(&format!("https://qiita.com/api/v2/items/:{}", id))
            .await?;
        Ok(res)
    }
}
