use chrono::Utc;

use crate::database::entities::events::{EventModel, UPSEvent};

/// Seeds 50 AC failure events into the database
#[tokio::test]
#[ignore = "Seeding logic, not a real test"]
async fn seed_ac() {
    let db = crate::database::init().await;

    let start = Utc::now();

    for _ in 0..50 {
        let date = Utc::now();
        EventModel::create(&db, UPSEvent::ACFailure, date)
            .await
            .unwrap();
    }

    let end = Utc::now();

    let events = EventModel::get_range(&db, start, end).await.unwrap();
    dbg!(events);
}
