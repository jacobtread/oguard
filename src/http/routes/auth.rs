use crate::config::SharedConfig;
use crate::http::error::HttpResult;
use crate::http::error::HttpStatusResult;
use crate::http::models::{LoginRequest, LoginStateResponse};
use anyhow::anyhow;
use axum::{Extension, Json};
use axum_session::{ReadOnlySession, Session, SessionNullPool};
use reqwest::StatusCode;

/// GET /api/login-state
///
/// Responds with the users current login state
pub async fn login_state(
    session: ReadOnlySession<SessionNullPool>,
) -> HttpResult<LoginStateResponse> {
    let logged_in = session.get::<String>("login").is_some();

    Ok(Json(LoginStateResponse { logged_in }))
}

/// POST /api/login
///
/// Logs into the API so mutations can be made
pub async fn login(
    session: Session<SessionNullPool>,
    Extension(config): Extension<SharedConfig>,
    Json(LoginRequest { password }): Json<LoginRequest>,
) -> HttpStatusResult {
    // Get the credentials from the config
    let expected_password = match  config.login.password.as_ref() {
        Some(password ) => password,
        _ => return Err(anyhow!("authentication is not setup for this server. please set a password in the configuration file").into()),
    };

    // Password doesn't match
    if password.ne(expected_password) {
        return Err(anyhow!("the provided password is incorrect").into());
    }

    session.set("login", "true".to_string());
    session.set_store(true);

    Ok(StatusCode::OK)
}

/// POST /api/logout
///
/// Logs out the user clearing their session
pub async fn logout(session: Session<SessionNullPool>) -> HttpStatusResult {
    session.clear();
    Ok(StatusCode::OK)
}
