use sea_orm_migration::prelude::*;

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
                    .col(ColumnDef::new(Events::Type).integer().not_null())
                    .col(ColumnDef::new(Events::CreatedAt).date_time().not_null())
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
        // Drop the index
        manager
            .drop_index(
                Index::drop()
                    .table(Events::Table)
                    .name("idx-event-type")
                    .to_owned(),
            )
            .await?;

        // Drop the index
        manager
            .drop_index(
                Index::drop()
                    .table(Events::Table)
                    .name("idx-event-created-at")
                    .to_owned(),
            )
            .await?;

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
