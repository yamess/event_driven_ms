use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlobPayload {
    pub url: String,
    pub name: String,
    pub extension: String,
}

#[derive(Debug)]
pub struct StreamResponse<T: DeserializeOwned>{
    pub id: String,
    pub data: T
}