#[path = "Exceptions/mod.rs"]
pub mod exceptions;

// Optionally re-export specific types for easier access
pub use exceptions::{ApiError, ApiResult};