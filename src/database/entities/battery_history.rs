use crate::database::DbResult;
use crate::ups::DeviceBattery;
use futures::future::BoxFuture;
use sea_orm::entity::prelude::*;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DatabaseConnection,
};
use serde::Serialize;

pub type BatteryHistoryModel = Model;
pub type BatteryHistoryActiveModel = ActiveModel;
pub type BatteryHistoryEntity = Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "battery_history")]
pub struct Model {
    /// Unique ID for the event
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: i64,

    /// The state event
    pub state: DeviceBattery,

    /// Creation time for the event
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Creates a new player from the provided details
    pub fn create(
        db: &DatabaseConnection,
        state: DeviceBattery,
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
