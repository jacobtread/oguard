//! # HTTP
//!
//! Routing, models, middleware and handler functions for the HTTP server

pub mod error;
pub mod middleware;
pub mod models;
mod routes;

pub use routes::router;
