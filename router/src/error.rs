use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

// Define custom error types specific to the router module
#[derive(Debug)]
pub enum RouterError {
    // Define your custom error types here
    // Example:
    InvalidInput,
    ConnectionError,
    CacheError,
    InfoError,
    WarmupError,
    ArgumentValidationError(String),
    TokioError,
}

// Implement the error conversion trait for each custom error type
impl From<RouterError> for Box<dyn Error> {
    fn from(error: RouterError) -> Self {
        Box::new(error)
    }
}

// Implement the error display trait for each custom error type
impl Display for RouterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Implement the display format for each custom error type
        // Example:
        match self {
            RouterError::InvalidInput => write!(f, "Invalid input"),
            RouterError::ConnectionError => write!(f, "Connection error"),
            RouterError::CacheError => write!(f, "Cache error"),
            RouterError::InfoError => write!(f, "Info error"),
            RouterError::WarmupError => write!(f, "Warmup error"),
            RouterError::ArgumentValidationError(msg) => {
                write!(f, "Argument validation error: {}", msg)
            }
            RouterError::TokioError => write!(f, "Tokio error"),
        }
    }
}

// Write unit tests to ensure the correctness of the error handling logic
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_error_display() {
        // Test the display format of each custom error type
        // Example:
        assert_eq!(
            format!("{}", RouterError::InvalidInput),
            "Invalid input"
        );
        assert_eq!(
            format!("{}", RouterError::ConnectionError),
            "Connection error"
        );
        assert_eq!(format!("{}", RouterError::CacheError), "Cache error");
        assert_eq!(format!("{}", RouterError::InfoError), "Info error");
        assert_eq!(format!("{}", RouterError::WarmupError), "Warmup error");
        assert_eq!(
            format!("{}", RouterError::ArgumentValidationError("Invalid argument".to_string())),
            "Argument validation error: Invalid argument"
        );
        assert_eq!(format!("{}", RouterError::TokioError), "Tokio error");
    }
}
