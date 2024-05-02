use std::process;
use std::sync::{Arc, RwLock};
use std::thread::ThreadId;
use actix::{Actor, AsyncContext, Context, ContextFutureSpawner, Handler, ResponseFuture, SyncContext, WrapFuture};
use tokio::task::block_in_place;
use crate::interfaces::IStreaming;
use crate::redis::RedisStream;
use crate::schemas::{BlobPayload, StreamResult};
use crate::workers::messages::StartWorker;
use crate::errors::Error;
use crate::settings::RedisConfig;
use futures::executor::block_on;
use redis::ToRedisArgs;


#[derive(Debug, Clone)]
pub struct ClaimerWorker{
    pub config: RedisConfig,
    pub stream: String,
    pub group: String,
    pub consumer: String,
    pub min_idle: usize,
    pub start_id: String,
    pub count: usize,
}

impl Actor for ClaimerWorker{
    type Context = SyncContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("{} worker started", self.consumer);
        ctx.address().do_send(StartWorker);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Claimer worker stopped");
    }
}

impl Handler<StartWorker> for ClaimerWorker{
    type Result = ResponseFuture<()>;

    fn handle(&mut self, _msg: StartWorker, _ctx: &mut Self::Context) -> Self::Result {
        let mut client = RedisStream::new(self.config.clone()).unwrap();
        block_on(async move {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(3));
                let raw = client.auto_claim::<BlobPayload>(
                    self.stream.as_str(),
                    self.group.as_str(),
                    self.consumer.as_str(),
                    self.min_idle,
                    self.start_id.as_str(),
                    self.count
                );

                let payload = match raw {
                    Ok(payload) => payload,
                    Err(e) => {
                        log::error!("Failed to auto claim message: {:?}", e);
                        continue;
                    }
                };

                if payload.len() == 0 {
                    continue;
                }

                payload.iter().for_each(|p| {
                    let data: BlobPayload = p.data.clone();
                    let id: String = p.id.clone();
                    log::info!("{} processing message {} data: {:?}", self.consumer, id, data);
                    let ack = client.ack("stream", "group", &id);
                    match ack {
                        Ok(_) => log::info!("Message acknowledged"),
                        Err(e) => log::error!("Failed to acknowledge message: {:?}", e)
                    }
                });
            }
        })
    }
}