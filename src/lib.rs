use actix_web::HttpServer;

pub mod redis;
pub mod errors;
pub mod schemas;
pub mod logger;
pub mod settings;
pub mod interfaces;

async fn run_server() -> std::io::Result<()> {
    println!("Starting server...");

    HttpServer::new(|| {
        actix_web::App::new()
    });

    Ok(())
}
