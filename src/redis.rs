use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
use redis::{Commands, Connection, FromRedisValue, RedisResult};
use redis::streams::{StreamReadOptions, StreamReadReply};
use crate::schemas::{StreamPendingResult, StreamResult};
use crate::errors::{Error, Result};
use crate::settings::RedisConfig;
use serde::Serialize;
use crate::interfaces::{IStreaming};


pub struct RedisStream{
    conn: Connection
}

impl RedisStream{
    pub fn new(config: RedisConfig) -> Result<Self> {
        let client = redis::Client::open(config.redis_connection_string);
        let conn = match client {
            Ok(client) => match client.get_connection() {
                Ok(connection) => connection,
                Err(e) => {
                    log::error!("Failed to get connection from redis client: {:?}", e);
                    return Err(Error::RedisConnection(e.into()))
                }
            },
            Err(e) => {
                log::error!("Error connecting to Redis: {:?}", e);
                return Err(Error::RedisConnection(e.into()))
            }
        };

        Ok(Self { conn })
    }

    fn convert_from_redis<T>(&self, value: StreamReadReply) -> Result<Vec<StreamResult<T>>>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut payloads: Vec<StreamResult<T>> = Vec::new();

        let raw_payloads = value.keys[0].ids.clone();

        for raw in raw_payloads {
            let id = raw.id.clone();
            let data = raw.map.get("message").unwrap().clone();
            let data = match String::from_redis_value(&data) {
                Ok(data) => data,
                Err(e) => {
                    log::error!(
                        "Failed to parse message data with id: {}, data {:?}: {:?}",id,data,e
                    );
                    return Err(Error::Parsing(e.into()))
                }
            };
            let data = serde_json::from_str::<T>(&data).unwrap();
            payloads.push(StreamResult { id, data });
        }
        Ok(payloads)
    }
}

impl IStreaming for RedisStream{

    fn create_stream(&mut self, stream: &str) -> Result<()> {
        let id_result: RedisResult<String> = self.conn.xadd(
            stream, "*", &[("message", format!("stream {} created", stream))]
        );
        let id = match id_result {
            Ok(id) => id,
            Err(e) => {
                log::error!("Failed to create stream: {:?}", e);
                return Err(Error::AddStreamMessage(e.into()))
            }
        };
        let result = self.conn.xdel(stream, &[id.clone()]);
        match result {
            Ok(_) => log::info!("Stream {} created successfully", stream),
            Err(e) => {
                log::error!("Failed to delete message from stream creation: {:?}", e);
                return Err(Error::DeleteStreamMessage(e.into()))
            }
        }
    }
    fn create_group(&mut self, stream: &str, group: &str) {
        let _: () = match self.conn.xgroup_create(stream, group, "$") {
            Ok(_) => log::info!("Created consumer group {} for stream {}", group, stream),
            Err(e) => {
                log::warn!("Failed to create consumer group: {:?}", e);
            }
        };
    }

    fn add<P: Serialize + Send + Sync>(&mut self, stream: &str, payload: P) -> Result<()> {
        let payload = match serde_json::to_string(&payload){
            Ok(payload) => payload,
            Err(e) => {
                log::error!("Failed to serialize payload: {:?}", e);
                return Err(Error::Serialization(e.into()))
            }
        };
        let result: RedisResult<String> = self.conn.xadd(stream, "*", &[("message", payload)]);
        match result {
            Ok(id) => log::debug!("Added message {} to stream {}", id, stream),
            Err(e) => {
                log::error!("Failed to add message to stream: {:?}", e);
                return Err(Error::AddStreamMessage(e.into()))
            }
        }
        Ok(())
    }

    fn read<T>(
        &mut self, stream: &str, group: &str, consumer: &str, count: usize
    ) -> Result<Vec<StreamResult<T>>>
        where
            T: serde::de::DeserializeOwned,
    {
        let opts = StreamReadOptions::default()
            .group(group, consumer)
            .count(count)
            .block(0);
        let result: RedisResult<StreamReadReply> = self.conn.xread_options(
            &[stream], &[">"], &opts
        );
        let result = match result {
            Ok(result) => result,
            Err(e) => {
                log::error!("Failed to read from stream: {:?}", e);
                return Err(Error::ReadStreamMessage(e.into()))
            }
        };

        if result.keys.len() == 0 {
            return Ok(Vec::new());
        }

        let payloads = self.convert_from_redis::<T>(result)?;
        Ok(payloads)
    }

    fn ack(&mut self, stream: &str, group: &str, id: &str) -> Result<()> {
        let _: () = match self.conn.xack(stream, group, &[id]){
            Ok(_) => log::debug!("Acknowledged message with id {} from stream {}", id, stream),
            Err(e) => {
                log::error!("Failed to acknowledge message: {:?}", e);
                return Err(Error::AckStreamMessage(e.into()))
            }
        };
        Ok(())
    }

    fn pending(
        &mut self, stream: &str, group: &str, min_idle: usize, count: usize
    ) -> Result<Vec<StreamPendingResult>> {
        let result: RedisResult<Vec<Vec<(String, String, i64, i64)>>> = redis::Cmd::new()
            .arg("XPENDING")
            .arg(stream)
            .arg(group)
            .arg("IDLE")
            .arg(min_idle)
            .arg("-")
            .arg("+")
            .arg(count)
            .query(&mut self.conn);

        let result = match result {
            Ok(result) => result,
            Err(e) => {
                log::error!("Failed to get pending messages: {:?}", e);
                return Err(Error::PendingStreamMessage(e.into()))
            }
        };

        if result.len() == 0 {
            log::debug!("No pending messages in group {}", group);
            return Ok(Vec::new());
        }

        let mut response: Vec<StreamPendingResult> = Vec::new();

        let result = result[0].clone();
        for (id, consumer, idle, delivery_count) in result {
            response.push(StreamPendingResult {
                id: id.clone(),
                consumer: consumer.clone(),
                idle: idle as usize,
                delivery_count: delivery_count as usize
            });
        }
        Ok(response)
    }

    fn auto_claim<T>(
        &mut self,
        stream: &str,
        group: &str,
        consumer: &str,
        min_idle_time: usize,
        start_id: String,
        count: usize
    ) -> Result<Vec<StreamResult<T>>>
        where
            T: serde::de::DeserializeOwned,
    {
        let result: RedisResult<Vec<(String, Vec<Vec<(String, HashMap<String, String>)>>,
                                     Vec<String>)>> =
            redis::Cmd::new()
                .arg("XAUTOCLAIM")
                .arg(stream)
                .arg(group)
                .arg(consumer)
                .arg(min_idle_time)
                .arg(start_id.as_str())
                .arg("COUNT")
                .arg(count)
                .query(&mut self.conn);

        let result = match result {
            Ok(result) => result,
            Err(e) => {
                log::error!("Failed to claim messages from stream {}: {:?}", stream, e);
                return Err(Error::ClaimStreamMessage(e.into()))
            }
        };

        let results = result[0].1.clone();

        let mut res = Vec::new();
        for result in results {
            for (id, map) in result {
                let data = map.get("message").unwrap_or_default().clone();
                let data = String::from_str(&data).unwrap_or_default();
                let data = serde_json::from_str::<T>(&data)?;
                res.push(StreamResult { id, data });
            }
        }
        Ok(res)
    }

}