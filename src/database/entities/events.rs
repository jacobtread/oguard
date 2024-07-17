use crate::database::DbResult;
use futures::future::BoxFuture;
use sea_orm::entity::prelude::*;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DatabaseConnection,
};
use serde::{Deserialize, Serialize};
use strum::Display;

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
    pub ty: UPSEvent,

    /// Creation time for the event
    pub created_at: DateTimeUtc,
}

/// Events that could be encountered while processing state updates
#[derive(
    Debug, EnumIter, DeriveActiveEnum, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Display,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum UPSEvent {
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

impl UPSEvent {
    /// Defines the events that this event can cancel
    pub fn cancels(&self) -> &'static [UPSEvent] {
        match self {
            UPSEvent::ACFailure => &[UPSEvent::ACRecovery],
            UPSEvent::ACRecovery => &[UPSEvent::ACFailure],
            UPSEvent::UPSFault => &[],
            UPSEvent::LowBatteryModeStart => &[UPSEvent::LowBatteryModeEnd],
            UPSEvent::LowBatteryModeEnd => &[UPSEvent::LowBatteryModeStart],
            UPSEvent::BatteryTestStart => &[UPSEvent::BatteryTestEnd],
            UPSEvent::BatteryTestEnd => &[UPSEvent::BatteryTestStart],
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn create(
        db: &DatabaseConnection,
        ty: UPSEvent,
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
