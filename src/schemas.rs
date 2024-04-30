use redis::streams::{StreamClaimReply, StreamReadReply};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlobPayload {
    pub url: String,
    pub name: String,
    pub extension: String,
}

pub enum StreamResult{
    Read(StreamReadReply),
    Claim(StreamClaimReply),
}

#[derive(Debug)]
pub struct StreamResponse<T: DeserializeOwned>{
    pub id: String,
    pub data: T
}

