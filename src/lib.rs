use std::process;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use actix::{Actor, SyncArbiter};
use actix_web::{App, HttpServer, web};
use num_cpus;
use crate::app_state::AppState;
use crate::helpers::get_consumer_name;
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
pub mod helpers;

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
        let consumer = get_consumer_name("extractor".to_string());
        ExtractorWorker {
            stream,
            stream_name: "stream".to_string(),
            group: "group".to_string(),
            consumer,
            count: 1,
        }
    });

    let _ = SyncArbiter::start(3,  move || {
        let config = GlobalConfig::new();
        let consumer = get_consumer_name("claimer".to_string());
        ClaimerWorker {
            config: config.redis.clone(),
            stream: "stream".to_string(),
            group: "group".to_string(),
            consumer: consumer.clone(),
            min_idle: 3000,
            start_id: "0-0".to_string(),
            count: 1,
        }
    });


    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(state.clone())
            .service(handlers::produce)
            .service(handlers::healthz)
    })
        .bind(format!("{}:{}", server_config.host, server_config.port))?
        .workers(num_cpus::get_physical() - 1)
        .run()
        .await
}
