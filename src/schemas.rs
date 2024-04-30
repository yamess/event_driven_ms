use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlobPayload {
    pub url: String,
    pub name: String,
    pub extension: String,
}

pub struct StreamResponse{
    id: String,
    data: BlobPayload
}