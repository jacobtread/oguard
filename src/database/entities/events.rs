use crate::database::DbResult;
use crate::watcher::UPSEvent;
use futures::future::BoxFuture;
use sea_orm::entity::prelude::*;
use sea_orm::{prelude::*, EntityOrSelect};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DatabaseConnection,
};
use serde::Serialize;

pub type EventModel = Model;
pub type EventActiveModel = ActiveModel;
pub type EventEntity = Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "events")]
pub struct Model {
    /// Unique ID for the event
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: i64,

    /// Type of event that occurred
    #[sea_orm(column_name = "type")]
    #[serde(rename = "type")]
    pub ty: EventType,

    /// Creation time for the event
    pub created_at: DateTimeUtc,
}

#[derive(Debug, EnumIter, DeriveActiveEnum, PartialEq, Eq, Clone, Copy, Serialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum EventType {
    /// AC Power has been lost
    #[sea_orm(num_value = 0)]
    ACFailure,
    /// AC Power has been recovered
    #[sea_orm(num_value = 1)]
    ACRecovery,
    /// UPS has encountered a fault
    #[sea_orm(num_value = 2)]
    UPSFault,
    /// UPS has entered low battery mode
    #[sea_orm(num_value = 3)]
    LowBatteryModeStart,
    /// UPS has left low power mode
    #[sea_orm(num_value = 4)]
    LowBatteryModeEnd,
    /// UPS Battery test has started
    #[sea_orm(num_value = 5)]
    BatteryTestStart,
    /// UPS Battery test has ended
    #[sea_orm(num_value = 6)]
    BatteryTestEnd,
}

impl From<UPSEvent> for EventType {
    fn from(value: UPSEvent) -> Self {
        match value {
            UPSEvent::ACFailure => Self::ACFailure,
            UPSEvent::ACRecovery => Self::ACRecovery,
            UPSEvent::UPSFault => Self::UPSFault,
            UPSEvent::LowBatteryModeStart => Self::LowBatteryModeStart,
            UPSEvent::LowBatteryModeEnd => Self::LowBatteryModeEnd,
            UPSEvent::BatteryTestStart => Self::BatteryTestStart,
            UPSEvent::BatteryTestEnd => Self::BatteryTestEnd,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Creates a new player from the provided details
    pub fn create(
        db: &DatabaseConnection,
        ty: EventType,
        created_at: DateTimeUtc,
    ) -> BoxFuture<'_, DbResult<Self>> {
        ActiveModel {
            id: NotSet,
            ty: Set(ty),
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
