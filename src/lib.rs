use std::sync::Arc;
use actix::SyncArbiter;
use actix_web::{App, HttpServer, web};
use crate::app_state::AppState;
use crate::interfaces::IStreaming;
use crate::logger::init_logger;
use crate::redis::RedisStream;
use crate::settings::GlobalConfig;
use crate::workers::extractor::ExtractorWorker;

pub mod redis;
pub mod errors;
pub mod schemas;
pub mod logger;
pub mod settings;
pub mod interfaces;
mod workers;
mod app_state;

pub async fn run_server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let state = web::Data::new(AppState::new());
    init_logger(state.config.logger.clone());
    let server_config = state.config.server.clone();
    log::info!("Server started at http:://{}:{}!", server_config.host, server_config.port);

    // Create stream and groups
    let mut stream = RedisStream::new(state.config.redis.clone()).unwrap();
    //stream.create_stream("stream").unwrap();
    stream.create_group("stream", "group");

    let _ = SyncArbiter::start(1,  move || {
       let config = GlobalConfig::new();
        let stream = RedisStream::new(config.redis.clone()).unwrap();
        ExtractorWorker { stream }
    });

    HttpServer::new(move || {
        App::new()
    })
    .bind(format!("{}:{}", server_config.host, server_config.port))?
    .run()
    .await
}
