pub use sea_orm_migration::prelude::*;

mod m20240705_091507_create_events;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240705_091507_create_events::Migration)]
    }
}
