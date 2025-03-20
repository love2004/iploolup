pub mod ip;
pub mod ddns;

use actix_web::{web, HttpResponse, Responder};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/", web::get().to(index))
            .configure(ip::configure_routes)
            .configure(ddns::configure_routes)
    );
}

async fn index() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "IP Lookup API",
        "version": "1.0.0",
        "endpoints": {
            "ipv4": "/api/v1/ip/v4",
            "ipv6": "/api/v1/ip/v6",
            "ddns": {
                "ipv4": "/api/v1/ddns/update/ipv4",
                "ipv6": "/api/v1/ddns/update/ipv6"
            }
        }
    }))
}