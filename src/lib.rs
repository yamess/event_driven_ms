use actix_web::{App, HttpServer, web};
use crate::dependencies::AppState;
use crate::logger::init_logger;
use crate::settings::GlobalConfig;

pub mod redis;
pub mod errors;
pub mod schemas;
pub mod logger;
pub mod settings;
pub mod interfaces;
mod workers;
mod dependencies;

pub async fn run_server() -> std::io::Result<()> {
    let state = web::Data::new(AppState::new());
    init_logger(state.config.logger.clone());
    let server_config = state.config.server.clone();
    log::info!("Server started at http:://{}:{}!", server_config.host, server_config.port);

    HttpServer::new(move || {
        App::new()
    })
    .bind(format!("{}:{}", server_config.host, server_config.port))?
    .run()
    .await
}
