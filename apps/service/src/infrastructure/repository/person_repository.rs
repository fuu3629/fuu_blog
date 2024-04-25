use sea_orm::{ActiveValue, EntityTrait};

use crate::infrastructure::{
    entities::user, error::InfrastructureError, lib::establish_connection,
};
#[derive(Default)]
pub struct PersonRepository;

impl PersonRepository {
    pub async fn save(
        &self,
        user_id: &String,
        name: &String,
        hash_password: &String,
        qiita_api_key: &Option<String>,
        qiita_id: &Option<String>,
    ) -> Result<String, InfrastructureError> {
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
        let res = user::Entity::insert(user).exec(&db).await?;
        let user_id = res.last_insert_id;
        Ok(user_id)
    }

    pub async fn delete(&self, user_id: &String) -> Result<(), InfrastructureError> {
        let db = establish_connection().await?;
        user::Entity::delete_by_id(user_id).exec(&db).await?;
        Ok(())
    }

    pub async fn find_by_id(&self, user_id: &String) -> Result<user::Model, InfrastructureError> {
        let db = establish_connection().await?;
        let user = user::Entity::find_by_id(user_id).one(&db).await?.unwrap();
        Ok(user)
    }

    pub async fn find_all(&self) -> Result<Vec<user::Model>, InfrastructureError> {
        let db = establish_connection().await?;
        let users: Vec<user::Model> = user::Entity::find().all(&db).await?;
        Ok(users)
    }
}
