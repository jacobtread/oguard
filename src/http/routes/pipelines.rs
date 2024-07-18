use crate::{
    database::entities::event_pipeline::{EventPipelineId, EventPipelineModel, ListEventPipeline},
    http::{
        error::{HttpResult, HttpStatusResult},
        middleware::auth_gate::AuthGate,
        models::{CreateEventPipeline, UpdateEventPipeline},
    },
};
use anyhow::{anyhow, Context};
use axum::extract::Path;
use axum::{Extension, Json};
use axum_valid::Garde;
use chrono::Utc;
use hyper::StatusCode;
use sea_orm::DatabaseConnection;

/// GET /api/event-pipelines
///
/// Requests all the event pipelines
pub async fn get_event_pipelines(
    Extension(db): Extension<DatabaseConnection>,
) -> HttpResult<Vec<ListEventPipeline>> {
    let event_pipelines = EventPipelineModel::all(&db)
        .await
        .context("failed to query event pipelines")?;

    Ok(Json(event_pipelines))
}

/// GET /api/event-pipelines/:id
///
/// Requests a specific event pipeline
pub async fn get_event_pipeline(
    _: AuthGate,
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<EventPipelineId>,
) -> HttpResult<EventPipelineModel> {
    let event_pipeline = EventPipelineModel::find_by_id(&db, id)
        .await
        .context("failed to find event pipeline")?
        .ok_or(anyhow!("unknown event pipeline"))?;

    Ok(Json(event_pipeline))
}

/// PUT /api/event-pipelines/:id
///
/// Updates a event pipeline
pub async fn update_event_pipeline(
    _: AuthGate,
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<EventPipelineId>,
    Garde(Json(request)): Garde<Json<UpdateEventPipeline>>,
) -> HttpResult<EventPipelineModel> {
    let event_pipeline = EventPipelineModel::find_by_id(&db, id)
        .await
        .context("failed to find event pipeline")?
        .ok_or(anyhow!("unknown event pipeline"))?;

    let event_pipeline = event_pipeline
        .update(
            &db,
            request.name,
            request.event,
            request.pipeline,
            request.cancellable,
            request.enabled,
        )
        .await
        .context("failed to update pipeline")?;

    Ok(Json(event_pipeline))
}

/// POST /api/event-pipelines
///
/// Creates a new event pipeline
pub async fn create_event_pipeline(
    _: AuthGate,
    Extension(db): Extension<DatabaseConnection>,
    Garde(Json(request)): Garde<Json<CreateEventPipeline>>,
) -> HttpResult<EventPipelineModel> {
    let current_time = Utc::now();
    let event_pipeline = EventPipelineModel::create(
        &db,
        request.name,
        request.event,
        request.pipeline,
        request.cancellable,
        current_time,
    )
    .await
    .context("failed to find event pipeline")?;

    Ok(Json(event_pipeline))
}

/// DELETE /api/event-pipelines/:id
///
/// Deletes an event pipeline
pub async fn delete_event_pipeline(
    _: AuthGate,
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<EventPipelineId>,
) -> HttpStatusResult {
    let deleted = EventPipelineModel::delete(&db, id)
        .await
        .context("failed to find event pipeline")?;

    if !deleted {
        return Err(anyhow!("unknown event pipeline").into());
    }

    Ok(StatusCode::OK)
}
