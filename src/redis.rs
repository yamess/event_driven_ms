use redis::{Commands, Connection};
use redis::streams::{StreamReadOptions, StreamReadReply};
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

    pub fn read(&mut self, stream: &str, group: &str, consumer: &str) -> Result<StreamReadReply> {
        let opts = StreamReadOptions::default()
            .group(group, consumer)
            .count(3)
            .block(0);
        let result: StreamReadReply = self.conn.xread_options(&[stream], &[">"], &opts).unwrap();
        Ok(result)
    }

    pub fn ack(&mut self, stream: &str, group: &str, id: &str) -> Result<()> {
        let _: () = self.conn.xack(stream, group, &[id]).unwrap();
        println!("Acknowledged message with id {} from stream {}", id, stream);
        Ok(())
    }

    pub fn pending(&mut self, stream: &str, group: &str) -> Result<()> {
        let result: i64 = self.conn.xpending(stream, group).unwrap();
        println!("Pending messages in group {}: {}", group, result);
        Ok(())
    }
}

