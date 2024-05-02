use std::process;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use actix::{Actor, SyncArbiter};
use actix_web::{App, HttpServer, web};
use crate::app_state::AppState;
use crate::interfaces::IStreaming;
use crate::logger::init_logger;
use crate::redis::RedisStream;
use crate::settings::GlobalConfig;
use crate::workers::claimer::ClaimerWorker;
use crate::workers::extractor::ExtractorWorker;
use crate::workers::messages::StartWorker;

pub mod redis;
pub mod errors;
pub mod schemas;
pub mod logger;
pub mod settings;
pub mod interfaces;
mod workers;
mod app_state;
mod handlers;

pub async fn run_server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let state = web::Data::new(AppState::new());
    init_logger(state.config.logger.clone());
    let server_config = state.config.server.clone();
    log::info!("Server started at http:://{}:{}!", server_config.host, server_config.port);

    // Create stream and groups
    let mut stream = RedisStream::new(state.config.redis.clone()).unwrap();
    stream.create_group("stream", "group");

    let _ = SyncArbiter::start(3,  move || {
        let config = GlobalConfig::new();
        let stream = RedisStream::new(config.redis.clone()).unwrap();
        ExtractorWorker { stream }
    });

    let _ = SyncArbiter::start(3,  move || {
        let config = GlobalConfig::new();
        ClaimerWorker { config: config.redis.clone() }
    });

    // let addr = ClaimerWorker { config: state.config.redis.clone() }.start();
    // addr.do_send(StartWorker);

    let index = Arc::new(RwLock::new(0));
    let index_clone = index.clone();

    HttpServer::new(move || {
       let pr = process::id();
        log::info!("Workers process: {:?}", pr);
        App::new()
            .app_data(state.clone())
            .service(handlers::produce)
    })
    .bind(format!("{}:{}", server_config.host, server_config.port))?
    .run()
    .await
}
