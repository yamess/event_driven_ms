use actix_web::HttpServer;

mod redis;
mod errors;
mod schemas;

async fn run_server() -> std::io::Result<()> {
    println!("Starting server...");

    HttpServer::new(|| {
        actix_web::App::new()
    });

    Ok(())
}
