use crate::http::error::{DynHttpError, HttpError};
use anyhow::anyhow;
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_session::{ReadOnlySession, SessionNullPool};
use reqwest::StatusCode;
use thiserror::Error;

/// Extractor that will error when the user is not logged in
/// restricting a handler to only authenticated users
pub struct AuthGate;

#[derive(Debug, Error)]
#[error("Unauthorized")]
pub struct UnauthorizedError;

impl HttpError for UnauthorizedError {
    fn status(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}

impl<S> FromRequestParts<S> for AuthGate
where
    S: Send + Sync,
{
    type Rejection = DynHttpError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Request the current session
        let session: ReadOnlySession<SessionNullPool> =
            ReadOnlySession::from_request_parts(parts, state)
                .await
                .map_err(|_| anyhow!("missing session content"))?;
        // Request the session login field
        let login: Option<String> = session.get("login");

        // Non logged-in gets an error
        if login.is_none() {
            return Err(UnauthorizedError.into());
        }

        Ok(AuthGate)
    }
}
