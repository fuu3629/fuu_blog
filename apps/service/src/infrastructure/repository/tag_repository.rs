use sea_orm::{ActiveValue, EntityTrait};

use super::super::entities::tag;
use crate::infrastructure::{error::InfrastructureError, lib::establish_connection};

#[derive(Default)]
pub struct TagRepository;

impl TagRepository {
    pub async fn save(&self, post_id: i64, tags: &Vec<String>) -> Result<(), InfrastructureError> {
        let db = establish_connection().await?;
        let tags = tags
            .into_iter()
            .map(|tag| tag::ActiveModel {
                blog_id: ActiveValue::set(post_id),
                name: ActiveValue::set(tag.clone()),
                ..Default::default()
            })
            .collect::<Vec<tag::ActiveModel>>();
        tag::Entity::insert_many(tags).exec(&db).await?;
        Ok(())
    }
}
