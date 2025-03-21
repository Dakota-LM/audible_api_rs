// src/api/exceptions/mod.rs
mod error_exceptions;

// Re-export the public items from error_exceptions.rs
pub use error_exceptions::{ApiErrorException, ApiError, ApiResult};