use sea_orm_migration::prelude::*;

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
                    .col(ColumnDef::new(EventPipelines::Name).string().not_null())
                    .col(ColumnDef::new(EventPipelines::Event).integer().not_null())
                    .col(ColumnDef::new(EventPipelines::Pipelines).json().not_null())
                    .col(
                        ColumnDef::new(EventPipelines::Cancellable)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EventPipelines::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EventPipelines::ModifiedAt)
                            .date_time()
                            .not_null(),
                    )
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

        // Drop the index
        manager
            .drop_index(
                Index::drop()
                    .table(EventPipelines::Table)
                    .name("idx-event-pl-name")
                    .to_owned(),
            )
            .await?;

        // Drop the index
        manager
            .drop_index(
                Index::drop()
                    .table(EventPipelines::Table)
                    .name("idx-event-pl-event")
                    .to_owned(),
            )
            .await?;

        // Drop the index
        manager
            .drop_index(
                Index::drop()
                    .table(EventPipelines::Table)
                    .name("idx-event-pl-cancellable")
                    .to_owned(),
            )
            .await?;

        // Drop the index
        manager
            .drop_index(
                Index::drop()
                    .table(EventPipelines::Table)
                    .name("idx-event-pl-created-at")
                    .to_owned(),
            )
            .await?;
        // Drop the index
        manager
            .drop_index(
                Index::drop()
                    .table(EventPipelines::Table)
                    .name("idx-event-pl-modified-at")
                    .to_owned(),
            )
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
    Pipelines,
    Cancellable,
    CreatedAt,
    ModifiedAt,
}
