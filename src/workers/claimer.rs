use actix::{Actor, AsyncContext, Context, ContextFutureSpawner, Handler, ResponseFuture, SyncContext, WrapFuture};
use crate::interfaces::IStreaming;
use crate::redis::RedisStream;
use crate::schemas::{BlobPayload, StreamResult};
use crate::workers::messages::StartWorker;
use crate::errors::Error;
use crate::settings::RedisConfig;

#[derive(Debug, Clone)]
pub struct ClaimerWorker{
    pub config: RedisConfig
}

impl Actor for ClaimerWorker{
    type Context = SyncContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Claimer worker started");
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

        Box::pin(async move {
            loop {
                let raw = client.auto_claim::<BlobPayload>(
                    "stream",
                    "group",
                    "consumer",
                    5000,
                    "0-0",
                    1
                );

                let payload = match raw {
                    Ok(payload) => payload,
                    Err(e) => {
                        log::error!("Failed to auto claim message: {:?}", e);
                        continue;
                    }
                };

                if payload.len() == 0 {
                    log::info!("No messages to claim");
                    continue;
                }

                let data: BlobPayload = payload[0].data.clone();
                let id: String = payload[0].id.clone();

                log::info!("Auto Claimed message. Id: {} - Payload: {:?}", id, data);

                let ack = client.ack("stream", "group", &id);
                match ack {
                    Ok(_) => log::info!("Message acknowledged"),
                    Err(e) => log::error!("Failed to acknowledge message: {:?}", e)
                }
            }
        })

    }
}