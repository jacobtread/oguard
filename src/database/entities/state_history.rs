use crate::database::DbResult;
use crate::ups::DeviceState;
use futures::future::BoxFuture;
use sea_orm::entity::prelude::*;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DatabaseConnection,
};
use serde::Serialize;

pub type StateHistoryModel = Model;
pub type StateHistoryActiveModel = ActiveModel;
pub type StateHistoryEntity = Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "state_history")]
pub struct Model {
    /// Unique ID for the event
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: i64,

    /// The state event
    pub state: DeviceState,

    /// Creation time for the event
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn create(
        db: &DatabaseConnection,
        state: DeviceState,
        created_at: DateTimeUtc,
    ) -> BoxFuture<'_, DbResult<Self>> {
        ActiveModel {
            id: NotSet,
            state: Set(state),
            created_at: Set(created_at),
        }
        .insert(db)
    }

    pub async fn get_range(
        db: &DatabaseConnection,
        start: DateTimeUtc,
        end: DateTimeUtc,
    ) -> DbResult<Vec<Self>> {
        Entity::find()
            .filter(Column::CreatedAt.between(start, end))
            .all(db)
            .await
    }
}
