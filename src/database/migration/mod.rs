pub use sea_orm_migration::prelude::*;

mod m20240705_091507_create_events;
mod m20240706_034731_create_battery_history;
mod m20240706_034731_create_state_history;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240705_091507_create_events::Migration),
            Box::new(m20240706_034731_create_state_history::Migration),
            Box::new(m20240706_034731_create_battery_history::Migration),
        ]
    }
}
