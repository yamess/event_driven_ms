use event_driven_ms::run_server;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    run_server().await
    // let mut handlers = Vec::new();

    // handlers.push(
    //     thread::spawn(move || {
    //         let mut stream = RedisStream::new();
    //
    //         let mut start_id = "0-0".to_string();
    //         loop {
    //
    //             thread::sleep(std::time::Duration::from_secs(3));
    //
    //             let result = stream.auto_claim::<BlobPayload>(
    //                 "stream",
    //                 "group",
    //                 "consumer",
    //                 3000,
    //                 start_id,
    //                 1
    //             ).unwrap();
    //
    //             start_id = result.0.clone();
    //
    //             if result.1.len() == 0 {
    //                 continue;
    //             }
    //
    //             let payload = result.1[0].data.clone();
    //             let id = result.1[0].id.clone();
    //             println!("Auto Claimed message. Id: {} - Payload: {:?}", result.0, payload);
    //
    //             // Acknowledge the message
    //             stream.ack("stream", "group", &id).unwrap();
    //
    //             // thread::sleep(std::time::Duration::from_secs(3));
    //         }
    //     })
    // );
    //
    // let mut stream = RedisStream::new();
    // let result = stream.pending("stream", "group", 3000, 15).unwrap();
    // let result = match result {
    //     StreamPendingReply::Data(data) => data,
    //     StreamPendingReply::Empty => {
    //         println!("No pending messages");
    //         return;
    //     }
    // };
    // println!("Number of Pending messages: {:?}", result.count);
    //
    // result.consumers.iter().for_each(|consumer| {
    //     println!("Consumer: {:?}", consumer);
    // });
    // println!("Auto Claiming messages ...!");
    // let result = stream.auto_claim::<BlobPayload>("stream", "group", "consumer", 1).unwrap();
    // println!("Auto Claimed messages: {:?}", result.len());
    // for pay in result {
    //     println!("id: {} map: {:?}", pay.id, pay.data);
    //     stream.ack("stream", "group", &pay.id).unwrap();
    // }
    //
    //
    // let result = stream.claim::<BlobPayload>("stream", "group", "consumer", 0, "0").unwrap();
    // println!("Claimed messages: {:?}", result);
    //
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
    //         thread::sleep(std::time::Duration::from_secs(1));
    //         let mut stream = RedisStream::new();
    //         // stream.create_stream("stream").unwrap();
    //         // stream.create_group("stream", "group").unwrap();
    //         stream.add("stream", payload.clone()).unwrap();
    //     }
    //
    // }));

    // handlers.push(thread::spawn(move || {
    //     loop {
    //         // thread::sleep(std::time::Duration::from_secs(3));
    //         let mut stream = RedisStream::new();
    //         let result: Vec<StreamResponse<BlobPayload>> = stream.read::<BlobPayload>(
    //             "stream",
    //             "group",
    //             "consumer_3",
    //             1
    //         ).unwrap();
    //
    //         let thread_id = thread::current().id();
    //         // for pay in result {
    //         //     println!("Thread id: {:?}, id: {} map: {:?}", thread_id, pay.id, pay.data);
    //         //     stream.ack("stream", "group", &pay.id).unwrap();
    //         // }
    //     }
    // }));

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
