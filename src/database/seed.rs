use chrono::Utc;

use crate::database::entities::events::{EventModel, UPSEvent};

#[tokio::test]
#[ignore = "Seeding logic, not a real test"]
async fn seed() {
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
