use std::thread;
use ::redis::streams::{StreamId, StreamReadReply};
use ::redis::Value;
use crate::redis::RedisStream;
use crate::schemas::{BlobPayload, StreamResponse};

mod redis;
mod errors;
mod schemas;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut handlers = Vec::new();



    //
    // handlers.push(thread::spawn(move || {
    //     let payload = BlobPayload {
    //         url: "https://www.google.com".to_string(),
    //         name: "google.html".to_string(),
    //         extension: "html".to_string(),
    //     };
    //     loop{
    //         thread::sleep(std::time::Duration::from_secs(1));
    //         let mut stream = RedisStream::new();
    //         // stream.create_stream("stream").unwrap();
    //         // stream.create_group("stream", "group").unwrap();
    //         stream.add("stream", payload.clone()).unwrap();
    //     }
    //
    // }));

    handlers.push(thread::spawn(move || {
        // loop{
            thread::sleep(std::time::Duration::from_secs(3));
            let mut stream = RedisStream::new();
            let result: StreamReadReply = stream.read("stream", "group", "consumer").unwrap();

            let raw_payloads = &result.keys[0].ids;

            let first: &StreamId = &raw_payloads[0];
            let id = first.id.clone();
            let data = first.map.clone();
            let data = data.get("message").unwrap().clone();

            println!("id: {}, map: {:?}", id, data);
            println!("Raw payloads: {:?}", raw_payloads);
            // stream.ack("stream", "group", &result.keys).unwrap();

        // }
    }));

    for handler in handlers {
        handler.join().unwrap();
    }

}
