use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BatteryHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BatteryHistory::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(json(BatteryHistory::State))
                    .col(date_time(BatteryHistory::CreatedAt))
                    .to_owned(),
            )
            .await?;

        // Create a index over the created at
        manager
            .create_index(
                Index::create()
                    .name("idx-battery-history-created-at")
                    .table(BatteryHistory::Table)
                    .col(BatteryHistory::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BatteryHistory::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum BatteryHistory {
    Table,
    Id,
    State,
    CreatedAt,
}
