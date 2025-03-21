use std::error::Error;
use std::fmt;
use url::Url;
use serde_json::Value;

// Base ApiErrorException type
#[derive(Debug)]
pub struct ApiErrorException {
    request_uri: String,
    json_object: Value,
    message: Option<String>,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl ApiErrorException {
    pub fn new(
        request_uri: impl Into<String>,
        json_object: Value,
        message: Option<String>,
        source: Option<Box<dyn Error + Send + Sync>>,
    ) -> Self {
        Self {
            request_uri: request_uri.into(),
            json_object,
            message,
            source,
        }
    }
    
    pub fn request_uri(&self) -> &str {
        &self.request_uri
    }
    
    pub fn json_object(&self) -> &Value {
        &self.json_object
    }
}

impl fmt::Display for ApiErrorException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "Audible API error: {} (URI: {})", msg, self.request_uri),
            None => write!(f, "Audible API error at URI: {}", self.request_uri),
        }
    }
}

impl Error for ApiErrorException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

// ApiError derived from ApiErrorException
#[derive(Debug)]
pub struct ApiError {
    inner: ApiErrorException,
}

impl ApiError {
    // Constructor for string URI
    pub fn new(
        request_uri: impl Into<String>,
        json_object: Value,
        message: Option<String>,
        source: Option<Box<dyn Error + Send + Sync>>,
    ) -> Self {
        Self {
            inner: ApiErrorException::new(request_uri, json_object, message, source),
        }
    }
    
    // Constructor for Url type
    pub fn from_url(
        request_uri: Url,
        json_object: Value,
        message: Option<String>,
        source: Option<Box<dyn Error + Send + Sync>>,
    ) -> Self {
        Self::new(request_uri.to_string(), json_object, message, source)
    }
}

// Implementing standard traits for ApiError
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API Error: {}", self.inner)
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner)
    }
}

// Example of how to use this in a Result context
pub type ApiResult<T> = Result<T, ApiError>;


#[cfg(test)]
mod error_exceptions_tests {
    use super::*;
    use serde_json::json;
    use url::Url;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_api_error_creation_and_display() {
        // Create a JSON object similar to what might be received from an API
        let json_response = json!({
            "error": {
                "code": "auth_failure",
                "message": "Authentication failed"
            },
            "request_id": "abc123"
        });
        
        // Create an ApiError using a string URI
        let error1 = ApiError::new(
            "https://api.example.com/v1/authenticate",
            json_response.clone(),
            Some("Failed to authenticate with the API".to_string()),
            None,
        );
        
        // Test the display implementation
        assert!(error1.to_string().contains("Failed to authenticate with the API"));
        assert!(error1.to_string().contains("API Error"));
        
        // Create an ApiError using a Url object and with an inner source error
        let url = Url::parse("https://api.example.com/v1/resources").unwrap();
        let inner_error = IoError::new(ErrorKind::ConnectionRefused, "Connection refused");
        let error2 = ApiError::from_url(
            url,
            json_response,
            Some("Failed to fetch resources".to_string()),
            Some(Box::new(inner_error)),
        );
        
        // Verify that the source error is preserved
        let source = error2.source().unwrap().downcast_ref::<ApiErrorException>().unwrap();
        assert!(source.request_uri().contains("api.example.com"));
        
        // Verify JSON content is preserved
        let json_obj = source.json_object();
        assert_eq!(json_obj["error"]["code"], "auth_failure");
    }
    
    #[test]
    fn test_api_result_usage() {
        // Define a function that returns an ApiResult
        fn fetch_data(should_succeed: bool) -> ApiResult<String> {
            if should_succeed {
                Ok("Data successfully retrieved".to_string())
            } else {
                let error_json = json!({"error": {"message": "Resource not found"}});
                Err(ApiError::new(
                    "https://api.example.com/resource/123",
                    error_json,
                    Some("Resource not found".to_string()),
                    None,
                ))
            }
        }
        
        // Test success case
        let result1 = fetch_data(true);
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), "Data successfully retrieved");
        
        // Test error case
        let result2 = fetch_data(false);
        assert!(result2.is_err());
        let err = result2.unwrap_err();
        assert!(err.to_string().contains("Resource not found"));
    }
}


