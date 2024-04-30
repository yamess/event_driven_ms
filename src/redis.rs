use redis::{Commands, Connection, FromRedisValue};
use redis::streams::{StreamPendingReply, StreamReadOptions, StreamReadReply};
use crate::schemas::StreamResponse;
use crate::errors::Result;
use serde::Serialize;

pub struct RedisStream{
    conn: Connection
}

impl RedisStream{
    pub fn new() -> Self {
        let client = redis::Client::open("redis://0.0.0.0:6379").unwrap();
        let conn = client.get_connection().unwrap();
        Self { conn }
    }

    pub fn create_group(&mut self, stream: &str, group: &str) -> Result<()> {
        let _: () = self.conn.xgroup_create(stream, group, "$").unwrap();
        println!("Created consumer group {} for stream {}", group, stream);
        Ok(())
    }

    pub fn create_stream(&mut self, stream: &str) -> Result<()> {
        let id: String = self.conn.xadd(
            stream, "*", &[("message", format!("stream {} created", stream))]
        ).unwrap();

        let res: usize = self.conn.xdel(stream, &[id.clone()]).unwrap();
        Ok(())
    }

    pub fn add<P: Serialize + Send + Sync>(&mut self, stream: &str, payload: P) -> Result<()> {
        let payload = serde_json::to_string(&payload).unwrap();
        let _: () = self.conn.xadd(stream, "*", &[("message", payload)]).unwrap();
        println!("Added message to stream {}", stream);
        Ok(())
    }

    pub fn read<T>(
        &mut self, stream: &str, group: &str, consumer: &str, count: usize
    ) -> Result<Vec<StreamResponse<T>>>
    where
        T: serde::de::DeserializeOwned,
    {
        let opts = StreamReadOptions::default()
            .group(group, consumer)
            .count(count)
            .block(0);
        let result: StreamReadReply = self.conn.xread_options(&[stream], &[">"], &opts).unwrap();
        let payloads = self.convert_from_redis::<T>(result).unwrap();
        Ok(payloads)
    }

    pub fn ack(&mut self, stream: &str, group: &str, id: &str) -> Result<()> {
        let _: () = self.conn.xack(stream, group, &[id]).unwrap();
        println!("Acknowledged message with id {} from stream {}", id, stream);
        Ok(())
    }

    pub fn pending(&mut self, stream: &str, group: &str) -> Result<StreamPendingReply> {
        let result: StreamPendingReply = self.conn.xpending(stream, group).unwrap();
        println!("Pending messages in group {}: {:?}", group, result);
        Ok(result)
    }

    fn convert_from_redis<T>(&self, value: StreamReadReply) -> Result<Vec<StreamResponse<T>>>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut payloads: Vec<StreamResponse<T>> = Vec::new();
        let raw_payloads = value.keys[0].ids.clone();
        for raw in raw_payloads {
            let id = raw.id.clone();
            let data = raw.map.get("message").unwrap().clone();
            let data = String::from_redis_value(&data).unwrap();
            let data = serde_json::from_str::<T>(&data).unwrap();
            payloads.push(StreamResponse { id, data });
        }
        Ok(payloads)
    }
}

