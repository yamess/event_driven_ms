use thiserror;
use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Failed serialization. Cause: {0}")]
    Serialization(String),
    #[error("Payload parsing failed. Cause: {0}")]
    Parsing(String),
    #[error("Redis connection failed. Cause: {0}")]
    RedisConnection(#[from] redis::RedisError),
    #[error("Failed to add to stream. Cause: {0}")]
    AddStreamMessage(String),
    #[error("Failed to read message from stream. Cause: {0}")]
    ReadStreamMessage(String),
    #[error("Failed to claim from stream. Cause: {0}")]
    ClaimStreamMessage(String),
    #[error("Failed to get pending messages. Cause: {0}")]
    PendingStreamMessage(String),
    #[error("Failed to delete stream message". Cause: {0})]
    DeleteStreamMessage(String),
    #[error("Failed to acknowledge message. Cause: {0}")]
    AckStreamMessage(String),
    #[error("Failed to create group. Cause: {0}")]
    CreateConsumerGroup(String),
    #[error("Failed to create stream. Cause: {0}")]
    CreateStream(String),
}


impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
