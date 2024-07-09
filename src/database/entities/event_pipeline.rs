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
use sea_orm::{FromJsonQueryResult, FromQueryResult, IntoActiveModel, QuerySelect};
use serde::{Deserialize, Serialize};

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

    /// The event this pipeline is for
    pub event: UPSEvent,

    /// Action pipelines for this event pipeline
    pub pipelines: DbActionPipelines,

    /// Whether the events that cancel this should abort the run
    pub cancellable: bool,

    /// Creation time for the event pipeline
    pub created_at: DateTimeUtc,
    /// When the pipeline was last updated
    pub modified_at: DateTimeUtc,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct DbActionPipelines(pub Vec<ActionPipeline>);

impl DbActionPipelines {
    #[inline]
    pub fn into_inner(self) -> Vec<ActionPipeline> {
        self.0
    }
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
        event: UPSEvent,
        pipelines: Vec<ActionPipeline>,
        cancellable: bool,
        created_at: DateTimeUtc,
    ) -> BoxFuture<'_, DbResult<Self>> {
        ActiveModel {
            id: NotSet,
            event: Set(event),
            pipelines: Set(DbActionPipelines(pipelines)),
            cancellable: Set(cancellable),
            created_at: Set(created_at),
            modified_at: Set(created_at),
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

    pub async fn update(
        self,
        db: &DatabaseConnection,
        pipelines: Vec<ActionPipeline>,
        cancellable: bool,
    ) -> DbResult<Self> {
        let mut active_model = self.into_active_model();
        active_model.pipelines = Set(DbActionPipelines(pipelines));
        active_model.cancellable = Set(cancellable);
        active_model.update(db).await
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