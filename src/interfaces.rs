use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::errors::Result;
use crate::schemas::{StreamPendingResult, StreamResult};

pub trait IStreaming: Send + Sync {
    fn create_stream(&mut self, stream: &str) -> Result<()>;
    fn create_group(&mut self, stream: &str, group: &str);
    fn add<P: Serialize + Send + Sync>(&mut self, stream: &str, payload: P) -> Result<()>;
    fn read<D: DeserializeOwned>(
        &mut self, stream: &str, group: &str, consumer: &str, count: usize
    ) -> Result<Vec<StreamResult<D>>>;
    fn ack(&mut self, stream: &str, group: &str, id: &str) -> Result<()>;
    fn pending(
        &mut self, stream: &str, group: &str, min_idle: usize, count: usize
    ) -> Result<Vec<StreamPendingResult>>;
    fn auto_claim<T: DeserializeOwned>(
        &mut self, stream: &str, group: &str, consumer: &str, min_idle: usize, start_id: &str, count: usize
    ) -> Result<Vec<StreamResult<T>>>;
}