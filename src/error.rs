use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct BmbpError {
    pub kind: String,
    pub code: String,
    pub message: String,
}

impl BmbpError {
    pub fn new<T>(kind: T, code: String, message: String) -> Self
    where
        T: ToString,
    {
        BmbpError {
            kind: kind.to_string(),
            code,
            message,
        }
    }
}

impl<T> From<T> for BmbpError
where
    T: std::error::Error,
{
    fn from(value: T) -> Self {
        BmbpError {
            message: value.to_string(),
            kind: "error".to_string(),
            code: "500".to_string(),
        }
    }
}
pub type BmbpResp<T> = Result<T, BmbpError>;
