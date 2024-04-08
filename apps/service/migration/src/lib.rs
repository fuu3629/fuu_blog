pub use sea_orm_migration::prelude::*;

mod m20240324_023411_menber;
mod m20240401_080245_blog;
mod m20240401_105921_tag;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240324_023411_menber::Migration),
            Box::new(m20240401_080245_blog::Migration),
            Box::new(m20240401_105921_tag::Migration),
        ]
    }
}
