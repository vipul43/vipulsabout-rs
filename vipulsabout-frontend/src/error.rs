//! Error type for error handling

use crate::types::ErrorInfo;
use thiserror::Error as ThisError;

/// Define all possible errors
#[derive(ThisError, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// 400
    #[error("Bad Request")]
    BadRequest,

    /// 401
    #[error("Unauthorized")]
    Unauthorized,

    /// 403
    #[error("Forbidden")]
    Forbidden,

    /// 404
    #[error("Not Found")]
    NotFound,

    /// 410
    #[error("Gone")]
    Gone,

    /// 422
    #[error("Unprocessable Entity: {0:?}")]
    UnprocessableEntity(ErrorInfo),

    /// 429
    #[error("Too Many Requests")]
    TooManyRequests,

    /// 500
    #[error("Internal Server Error")]
    InternalServerError,

    /// Serde deserialize error
    #[error("Deserialize Error")]
    DeserializeError,

    /// Request error
    #[error("Http Request Error")]
    RequestError,
}
