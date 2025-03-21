// src/lib.rs

// Declare the api module
pub mod api;

// Optionally re-export error types at the crate root for convenience
pub use api::exceptions::{ApiError, ApiResult};

// Rest of your lib.rs content

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
