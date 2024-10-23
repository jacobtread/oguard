//! # Utils
//!
//! Utility modules and windows platform specific service code

pub mod validate;

#[cfg(target_os = "windows")]
pub mod windows_service;
