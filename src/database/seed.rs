use chrono::Utc;

use crate::database::{
    connect_database,
    entities::events::{EventModel, EventType},
};

#[tokio::test]
#[ignore = "Seeding logic, not a real test"]
async fn seed() {
    let db = connect_database().await;

    let start = Utc::now();

    for _ in 0..50 {
        let date = Utc::now();
        EventModel::create(&db, EventType::ACFailure, date)
            .await
            .unwrap();
    }

    let end = Utc::now();

    let events = EventModel::get_range(&db, start, end).await.unwrap();
    dbg!(events);
}
