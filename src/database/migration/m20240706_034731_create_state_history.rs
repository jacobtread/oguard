use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StateHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StateHistory::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(StateHistory::State).json().not_null())
                    .col(
                        ColumnDef::new(StateHistory::CreatedAt)
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
                    .name("idx-state-history-created-at")
                    .table(StateHistory::Table)
                    .col(StateHistory::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StateHistory::Table).to_owned())
            .await?;
        // Drop the index
        manager
            .drop_index(
                Index::drop()
                    .table(StateHistory::Table)
                    .name("idx-state-history-created-at")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum StateHistory {
    Table,
    Id,
    State,
    CreatedAt,
}
