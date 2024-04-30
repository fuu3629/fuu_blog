use super::{super::entities::post, tag_repository::TagRepository};
use crate::{
    domain::entity::post::Post,
    infrastructure::{entities::tag, error::InfrastructureError, lib::establish_connection},
};
use futures::{stream, stream::StreamExt};
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

#[derive(Default)]
pub struct PostRepository {
    tag_repository: TagRepository,
}

impl PostRepository {
    pub async fn save(&self, user_id: &String, blogs: Post) -> Result<i64, InfrastructureError> {
        let db = establish_connection().await?;
        let post = post::ActiveModel {
            user_id: ActiveValue::set(user_id.to_string()),
            created_at: ActiveValue::set(blogs.created_at),
            title: ActiveValue::set(blogs.title),
            body: ActiveValue::set(blogs.body),
            ..Default::default()
        };
        let blog_id = post::Entity::insert(post).exec(&db).await?.last_insert_id;
        self.tag_repository.save(blog_id, &blogs.tags).await?;

        Ok(blog_id)
    }

    pub async fn find_by_user_ids(
        &self,
        ids: Vec<String>,
        page: &i32,
        page_size: &i32,
        order: &i32,
    ) -> Result<(Vec<(post::Model, Vec<tag::Model>)>, u64), InfrastructureError> {
        let offset = ((page - 1) * page_size) as u64;
        let limit = page_size.clone() as u64;
        let db = establish_connection().await?;
        let total_count = post::Entity::find()
            .filter(post::Column::UserId.is_in(&ids))
            .count(&db)
            .await?;
        let slect_blog = post::Entity::find()
            .offset(offset)
            .filter(post::Column::UserId.is_in(ids));
        let order_blogs = match order {
            0 => slect_blog.order_by_asc(post::Column::CreatedAt),
            1 => slect_blog.order_by_desc(post::Column::CreatedAt),
            _ => slect_blog.order_by_asc(post::Column::CreatedAt),
        };

        let ordered_blogs_with_limit = order_blogs.limit(limit);
        let blogs = ordered_blogs_with_limit.all(&db).await?;
        let tmp2 = stream::iter(blogs)
            .then(|blog| {
                let db_ = &db;
                async move {
                    (
                        blog.clone(),
                        blog.find_related(tag::Entity).all(db_).await.unwrap(),
                    )
                }
            })
            .collect::<Vec<(post::Model, Vec<tag::Model>)>>()
            .await;
        Ok((tmp2, total_count))
    }

    pub async fn find_by_id(
        &self,
        blog_id: &i64,
    ) -> Result<(post::Model, Vec<tag::Model>), InfrastructureError> {
        let db = establish_connection().await?;
        let blog = post::Entity::find_by_id(*blog_id)
            .find_with_related(tag::Entity)
            .all(&db)
            .await?;
        Ok(blog[0].clone())
    }
}
