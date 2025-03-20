use actix_web::{web, HttpResponse, Responder};
use crate::services::ip;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ip")
            .route("/v4", web::get().to(get_ipv4))
            .route("/v6", web::get().to(get_ipv6))
    );
}

async fn get_ipv4() -> impl Responder {
    match ip::fetch_ipv4().await {
        Ok(ip) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "data": {
                "ip": ip
            }
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": e.to_string()
        }))
    }
}

async fn get_ipv6() -> impl Responder {
    match ip::fetch_ipv6().await {
        Ok(ip) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "data": {
                "ip": ip
            }
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": e.to_string()
        }))
    }
}