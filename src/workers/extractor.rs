use std::thread::Thread;
use actix::{Actor, Handler, SyncContext};
use log4rs::Handle;
use crate::errors::Result;
use crate::interfaces::IStreaming;
use crate::redis::RedisStream;
use crate::schemas::{BlobPayload, StreamResult};
use crate::workers::messages::StartWorker;

pub struct ExtractorWorker {
    pub stream: RedisStream,
    pub stream_name: String,
    pub group: String,
    pub consumer: String,
    pub count: usize,
}

impl Actor for ExtractorWorker {
    type Context = SyncContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("{} worker started!", self.consumer);
        ctx.address().do_send(StartWorker);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Extractor worker stopped!");
    }
}

impl Handler<StartWorker> for ExtractorWorker {
    type Result = ();

    fn handle(&mut self, _msg: StartWorker, _ctx: &mut Self::Context) -> Self::Result {
        loop {
            let payload: Result<Vec<StreamResult<BlobPayload>>> = self.stream.read(
                self.stream_name.as_str(),
                self.group.as_str(),
                self.consumer.as_str(),
                self.count
            );
            let payload = match payload {
                Ok(payload) => {
                    if payload.len() == 0 {
                        log::info!("No messages to process");
                        continue;
                    }
                    payload
                },
                Err(e) => {
                    log::error!("Failed to read message from stream: {:?}", e);
                    continue;
                }
            };

            payload.iter().for_each(|p| {
                let data = p.data.clone();
                let id = p.id.clone();

                // Processing of the data step goes here...
                log::info!("{} processing message {} data: {:?}", self.consumer, id, data);

                // Now tha the data has been processed, we can acknowledge the message
                let result= self.stream.ack("stream", "group", id.as_str());
                match result {
                    Ok(_) => log::info!("Message acknowledged"),
                    Err(e) => log::error!("Failed to acknowledge message: {:?}", e)
                }
            });
        }
    }
}