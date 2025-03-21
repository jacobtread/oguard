use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Events::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Events::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(integer(Events::Type))
                    .col(date_time(Events::CreatedAt))
                    .to_owned(),
            )
            .await?;

        // Create a index over the type
        manager
            .create_index(
                Index::create()
                    .name("idx-event-type")
                    .table(Events::Table)
                    .col(Events::Type)
                    .to_owned(),
            )
            .await?;

        // Create a index over the created at
        manager
            .create_index(
                Index::create()
                    .name("idx-event-created-at")
                    .table(Events::Table)
                    .col(Events::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Events::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Events {
    Table,
    Id,
    Type,
    CreatedAt,
}
