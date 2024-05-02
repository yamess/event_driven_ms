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
}

impl Actor for ExtractorWorker {
    type Context = SyncContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Extractor worker started!");
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
            let th = std::thread::current();
            log::info!("Processing from Thread id: {:?}", th.id());
            let payload: Result<Vec<StreamResult<BlobPayload>>> = self.stream.read(
                "stream",
                "group",
                "consumer",
                1
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
            let result = payload[0].data.clone();
            let id = payload[0].id.clone();
            log::info!("Thread: {:?} - Processing payload: {:?}", th.id(), result);

            let result= self.stream.ack("stream", "group", id.as_str());
            match result {
                Ok(_) => log::info!("Message acknowledged"),
                Err(e) => log::error!("Failed to acknowledge message: {:?}", e)
            }

            //std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}