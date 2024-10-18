use sea_orm_migration::prelude::*;

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
                    .col(ColumnDef::new(BatteryHistory::State).json().not_null())
                    .col(
                        ColumnDef::new(BatteryHistory::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
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
        // Drop the index
        manager
            .drop_index(
                Index::drop()
                    .table(BatteryHistory::Table)
                    .name("idx-battery-history-created-at")
                    .to_owned(),
            )
            .await?;

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
