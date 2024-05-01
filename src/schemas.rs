use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlobPayload {
    pub url: String,
    pub name: String,
    pub extension: String,
}

#[derive(Debug)]
pub struct StreamResult<T: DeserializeOwned>{
    pub id: String,
    pub data: T
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamPendingResult{
    pub id: String,
    pub consumer: String,
    pub idle: usize,
    pub delivery_count: usize
}