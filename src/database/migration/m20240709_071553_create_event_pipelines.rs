use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EventPipelines::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EventPipelines::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(string(EventPipelines::Name))
                    .col(integer(EventPipelines::Event))
                    .col(json(EventPipelines::Pipeline))
                    .col(boolean(EventPipelines::Cancellable))
                    .col(boolean(EventPipelines::Enabled))
                    .col(date_time(EventPipelines::CreatedAt))
                    .col(date_time(EventPipelines::ModifiedAt))
                    .col(date_time(EventPipelines::LastExecutedAt))
                    .to_owned(),
            )
            .await?;

        // Create a index over the name
        manager
            .create_index(
                Index::create()
                    .name("idx-event-pl-name")
                    .table(EventPipelines::Table)
                    .col(EventPipelines::Name)
                    .to_owned(),
            )
            .await?;
        // Create a index over the type
        manager
            .create_index(
                Index::create()
                    .name("idx-event-pl-event")
                    .table(EventPipelines::Table)
                    .col(EventPipelines::Event)
                    .to_owned(),
            )
            .await?;

        // Create a index over the type
        manager
            .create_index(
                Index::create()
                    .name("idx-event-pl-cancellable")
                    .table(EventPipelines::Table)
                    .col(EventPipelines::Cancellable)
                    .to_owned(),
            )
            .await?;

        // Create a index over the created at
        manager
            .create_index(
                Index::create()
                    .name("idx-event-pl-created-at")
                    .table(EventPipelines::Table)
                    .col(EventPipelines::CreatedAt)
                    .to_owned(),
            )
            .await?; // Create a index over the created at
        manager
            .create_index(
                Index::create()
                    .name("idx-event-pl-modified-at")
                    .table(EventPipelines::Table)
                    .col(EventPipelines::ModifiedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EventPipelines::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum EventPipelines {
    Table,
    Id,
    Name,
    Event,
    Pipeline,
    Cancellable,
    Enabled,
    CreatedAt,
    ModifiedAt,
    LastExecutedAt,
}
