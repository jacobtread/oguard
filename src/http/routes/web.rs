use axum::{
    body::Body,
    http::{HeaderValue, Request},
    response::{IntoResponse, Response},
};
use embeddy::Embedded;
use futures::future::BoxFuture;
use hyper::{header::CONTENT_TYPE, StatusCode};
use std::{
    convert::Infallible,
    path::{Path, PathBuf},
    task::{Context, Poll},
};
use tower::Service;

/// Embedded resources for the web app
///
/// TODO: This may not be particularly performant with a match statement
/// over all the public assets
#[derive(Clone, Embedded)]
#[folder = "src/resources/web"]
pub struct PublicContent;

fn find_local_path(path: &str) -> Option<PathBuf> {
    let data_path = Path::new("data/public").canonicalize().ok()?;
    let file_path = data_path.join(path).canonicalize().ok()?;
    // Folders outside of the data path should be ignored
    if !file_path.starts_with(data_path) {
        return None;
    }

    Some(file_path)
}

impl Service<Request<Body>> for PublicContent {
    type Response = Response;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let path = req.uri().path();

        // Strip the leading slash in order to match paths correctly
        let mut path = match path.strip_prefix('/') {
            Some(value) => value.to_string(),
            None => path.to_string(),
        };

        let std_path = Path::new(&path);

        // Determine type using extension
        let extension: String = match std_path.extension() {
            // Extract the extension
            Some(value) => value.to_string_lossy().to_string(),
            // Use the index file when responding to paths (For SPA dashboard support)
            None => {
                path = "index.html".to_string();
                "html".to_string()
            }
        };

        Box::pin(async move {
            let path = path;

            let mime_guess = mime_guess::from_ext(&extension);
            let mime_type = mime_guess.first_raw().unwrap_or("text/plain");

            let mime_header = HeaderValue::from_str(mime_type);

            // File exists in public data folder server try serve that and fallback to next on failure
            if let Some(local_path) = find_local_path(&path) {
                if local_path.exists() && local_path.is_file() {
                    if let Ok(contents) = tokio::fs::read(local_path).await {
                        // Create byte response from the embedded file
                        let mut response = Body::from(contents).into_response();

                        if let Ok(mime_header) = mime_header {
                            response.headers_mut().insert(CONTENT_TYPE, mime_header);
                        }

                        return Ok(response);
                    }
                }
            }

            // File exists within binary serve that
            if let Some(contents) = Self::get(&path) {
                // Create byte response from the embedded file
                let mut response = Body::from(contents).into_response();

                if let Ok(mime_header) = mime_header {
                    response.headers_mut().insert(CONTENT_TYPE, mime_header);
                }

                return Ok(response);
            }

            // All above failed server 404
            Ok(StatusCode::NOT_FOUND.into_response())
        })
    }
}
