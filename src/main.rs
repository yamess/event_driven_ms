use std::thread;
use ::redis::streams::{StreamId, StreamPendingReply, StreamReadReply};
use ::redis::{FromRedisValue, Value};
use crate::redis::RedisStream;
use crate::schemas::{BlobPayload, StreamResponse};

mod redis;
mod errors;
mod schemas;


#[tokio::main]
async fn main() {
    println!("Server started ...!");

    let mut stream = RedisStream::new();
    let result: StreamPendingReply = stream.pending("stream", "group").unwrap();
    let result = match result {
        StreamPendingReply::Data(data) => data,
        StreamPendingReply::Empty => {
            println!("No pending messages");
            return;
        }
    };
    println!("Pending messages: {:?}", result);
    result.consumers.iter().for_each(|consumer| {
        println!("Consumer: {:?}", consumer);
    });
    // let data = result.start_id;
    // let mut handlers = Vec::new();
    //
    // handlers.push(thread::spawn(move || {
    //     let payload = BlobPayload {
    //         url: "https://www.google.com".to_string(),
    //         name: "google.html".to_string(),
    //         extension: "html".to_string(),
    //     };
    //     loop{
    //         thread::sleep(std::time::Duration::from_secs(3));
    //         let mut stream = RedisStream::new();
    //         // stream.create_stream("stream").unwrap();
    //         // stream.create_group("stream", "group").unwrap();
    //         stream.add("stream", payload.clone()).unwrap();
    //     }
    //
    // }));
    //
    // handlers.push(thread::spawn(move || {
    //     loop {
    //         // thread::sleep(std::time::Duration::from_secs(3));
    //         let mut stream = RedisStream::new();
    //         let result: Vec<StreamResponse<BlobPayload>> = stream.read::<BlobPayload>(
    //             "stream",
    //             "group",
    //             "consumer",
    //             1
    //         ).unwrap();
    //
    //         let thread_id = thread::current().id();
    //         for pay in result {
    //             println!("Thread id: {:?}, id: {} map: {:?}", thread_id, pay.id, pay.data);
    //             stream.ack("stream", "group", &pay.id).unwrap();
    //         }
    //     }
    // }));
    //
    // handlers.push(thread::spawn(move || {
    //     loop {
    //         // thread::sleep(std::time::Duration::from_secs(3));
    //         let mut stream = RedisStream::new();
    //         let result: Vec<StreamResponse<BlobPayload>> = stream.read::<BlobPayload>(
    //             "stream",
    //             "group",
    //             "consumer",
    //             1
    //         ).unwrap();
    //         let thread_id = thread::current().id();
    //         for pay in result {
    //             println!("Thread id: {:?}, id: {} map: {:?}", thread_id, pay.id, pay.data);
    //             stream.ack("stream", "group", &pay.id).unwrap();
    //         }
    //     }
    // }));
    //
    // for handler in handlers {
    //     handler.join().unwrap();
    // }

}
