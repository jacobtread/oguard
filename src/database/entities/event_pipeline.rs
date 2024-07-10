use crate::action::ActionPipeline;
use crate::database::DbResult;
use axum::async_trait;
use chrono::Utc;
use futures::future::BoxFuture;
use sea_orm::entity::prelude::*;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DatabaseConnection,
};
use sea_orm::{FromQueryResult, IntoActiveModel, QuerySelect};
use serde::Serialize;

use super::events::UPSEvent;

pub type EventPipelineId = i64;
pub type EventPipelineModel = Model;
pub type EventPipelineActiveModel = ActiveModel;
pub type EventPipelineEntity = Entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "event_pipelines")]
pub struct Model {
    /// Unique ID for the event
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: i64,

    /// User provided name for the pipeline
    pub name: String,

    /// The event this pipeline is for
    pub event: UPSEvent,

    /// Pipeline of actions to run
    pub pipeline: ActionPipeline,

    /// Whether the events that cancel this should abort the run
    pub cancellable: bool,

    /// Whether the pipeline is enabled
    pub enabled: bool,

    /// Creation time for the event pipeline
    pub created_at: DateTimeUtc,
    /// When the pipeline was last updated
    pub modified_at: DateTimeUtc,
    /// When the pipeline was last executed
    pub last_executed_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// Will be called before `ActiveModel::insert`, `ActiveModel::update`, and `ActiveModel::save`
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // Update last modified time
        if !insert {
            self.modified_at = Set(Utc::now());
        }
        Ok(self)
    }
}

#[derive(FromQueryResult)]
pub struct CancellableEventPipeline {
    /// Unique ID for the event
    pub id: i64,

    /// The event this pipeline is for
    pub event: UPSEvent,

    /// Whether the events that cancel this should abort the run
    pub cancellable: bool,
}

impl Model {
    /// Creates a new player from the provided details
    pub fn create(
        db: &DatabaseConnection,
        name: String,
        event: UPSEvent,
        pipeline: ActionPipeline,
        cancellable: bool,
        created_at: DateTimeUtc,
    ) -> BoxFuture<'_, DbResult<Self>> {
        ActiveModel {
            id: NotSet,
            name: Set(name),
            event: Set(event),
            pipeline: Set(pipeline),
            cancellable: Set(cancellable),
            enabled: Set(true),
            created_at: Set(created_at),
            modified_at: Set(created_at),
            last_executed_at: Set(None),
        }
        .insert(db)
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: EventPipelineId,
    ) -> DbResult<Option<Self>> {
        Entity::find_by_id(id).one(db).await
    }

    pub async fn all(db: &DatabaseConnection) -> DbResult<Vec<Self>> {
        Entity::find().all(db).await
    }

    pub async fn find_by_event(db: &DatabaseConnection, event: UPSEvent) -> DbResult<Vec<Self>> {
        Entity::find().filter(Column::Event.eq(event)).all(db).await
    }

    pub async fn find_by_event_enabled(
        db: &DatabaseConnection,
        event: UPSEvent,
    ) -> DbResult<Vec<Self>> {
        Entity::find()
            .filter(Column::Event.eq(event).and(Column::Enabled.eq(true)))
            .all(db)
            .await
    }

    pub async fn update(
        self,
        db: &DatabaseConnection,
        name: String,
        pipeline: ActionPipeline,
        cancellable: bool,
        enabled: bool,
    ) -> DbResult<Self> {
        let mut active_model = self.into_active_model();
        active_model.name = Set(name);
        active_model.pipeline = Set(pipeline);
        active_model.cancellable = Set(cancellable);
        active_model.enabled = Set(enabled);
        active_model.update(db).await
    }

    pub async fn set_last_executed(
        db: &DatabaseConnection,
        id: EventPipelineId,
        last_executed_at: DateTimeUtc,
    ) -> DbResult<()> {
        Entity::update_many()
            .col_expr(Column::LastExecutedAt, Expr::value(Some(last_executed_at)))
            .filter(Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(())
    }

    /// Finds cancellable pipelines for the provided events
    pub async fn find_cancellable(
        db: &DatabaseConnection,
        events: Vec<UPSEvent>,
    ) -> DbResult<Vec<CancellableEventPipeline>> {
        Entity::find()
            .select_only()
            .column(Column::Id)
            .column(Column::Event)
            .column(Column::Cancellable)
            .filter(
                Column::Event
                    .is_in(events)
                    .and(Column::Cancellable.eq(true)),
            )
            .into_model::<CancellableEventPipeline>()
            .all(db)
            .await
    }
}
