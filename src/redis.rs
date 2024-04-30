use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
use redis::{Commands, Connection, FromRedisValue, RedisResult};
use redis::streams::{StreamClaimOptions, StreamClaimReply, StreamPendingReply, StreamReadOptions, StreamReadReply};
use crate::schemas::{StreamResponse, StreamResult};
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
        let result = StreamResult::Read(result);
        let payloads = self.convert_from_redis::<T>(result).unwrap();
        Ok(payloads)
    }

    pub fn ack(&mut self, stream: &str, group: &str, id: &str) -> Result<()> {
        let _: () = self.conn.xack(stream, group, &[id]).unwrap();
        println!("Acknowledged message with id {} from stream {}", id, stream);
        Ok(())
    }

    pub fn pending(
        &mut self, stream: &str, group: &str, min_idle_time: usize
    ) -> Result<()> {
        let result: Vec<Vec<(String, String, i64, i64)>> = redis::Cmd::new()
            .arg("XPENDING")
            .arg(stream)
            .arg(group)
            .arg("IDLE")
            .arg(min_idle_time)
            .arg("-")
            .arg("+")
            .arg(1000)
            .arg("consumer")
            .query(&mut self.conn)
            .unwrap();

        println!("Count: {:?}", result.len());
        println!("Pending messages in group {}: {:?}", group, result);
        Ok(())
    }

    pub fn claim<T>(
        &mut self, stream: &str, group: &str, consumer: &str, min_idle_time: usize, start_id: &str
    ) -> Result<Vec<StreamResponse<T>>>
    where
        T: serde::de::DeserializeOwned,
    {
        let result: StreamClaimReply = self.conn.xclaim(
            stream, group, consumer, min_idle_time, &[start_id]
        ).unwrap();

        println!("Claimed messages: {:?}", result);
        let result = StreamResult::Claim(result);
        let payloads = self.convert_from_redis::<T>(result).unwrap();
        Ok(payloads)
    }

    pub fn auto_claim<T>(
        &mut self, stream: &str, group: &str, consumer: &str, min_idle_time: usize,
        start_id: String,
        count: usize
    ) -> Result<(String, Vec<StreamResponse<T>>)>
    where
        T: serde::de::DeserializeOwned,
    {
        let result: Vec<(String, Vec<Vec<(String, HashMap<String, String>)>>, Vec<String>)> =
            redis::Cmd::new()
            .arg("XAUTOCLAIM")
            .arg(stream)
            .arg(group)
            .arg(consumer)
            .arg(min_idle_time)
            .arg(start_id.as_str())
            .arg("COUNT")
            .arg(count)
            .query(&mut self.conn)
            .unwrap();

        let next_start_id = result[0].0.clone();
        let results = result[0].1.clone();

        let mut res = Vec::new();
        for result in results {
            for (id, map) in result {
                let data = map.get("message").unwrap().clone();
                let data = String::from_str(&data).unwrap();
                let data = serde_json::from_str::<T>(&data).unwrap();
                res.push(StreamResponse { id, data });
            }
        }
        Ok((next_start_id, res))
    }

    fn convert_from_redis<T>(&self, value: StreamResult) -> Result<Vec<StreamResponse<T>>>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut payloads: Vec<StreamResponse<T>> = Vec::new();
        // let raw_payloads = value.keys[0].ids.clone();

        let raw_payloads = match value {
            StreamResult::Read(reply) => reply.keys[0].ids.clone(),
            StreamResult::Claim(reply) => reply.ids.clone(),
        };

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

