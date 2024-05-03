use actix_web::{get, HttpResponse, post, Responder, web};
use crate::app_state::AppState;
use crate::interfaces::IStreaming;
use crate::schemas::BlobPayload;
use crate::errors::Result;
use crate::redis::RedisStream;

#[post("/producer")]
async fn produce(data: web::Data<AppState>) -> Result<impl Responder> {
    let mut stream = RedisStream::new(data.config.redis.clone()).unwrap();
    let payload = BlobPayload {
        url: "http://localhost:9090".to_string(),
        name: "localhost".to_string(),
        extension: "html".to_string(),
    };
    let result = stream.add("stream", payload);
    match result {
        Ok(_) => {
            log::info!("Message added to stream");
            Ok(HttpResponse::Ok().json("Message added to stream"))
        },
        Err(e) => {
            log::error!("Failed to add message to stream: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to add message to stream"))
        }
    }

}

#[get("/healthz")]
async fn healthz() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json("OK"))
}