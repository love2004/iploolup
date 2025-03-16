pub mod ip;

use actix_web::{web, HttpResponse, Responder};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // 添加首頁路由
    cfg.route("/", web::get().to(index));
    
    // 配置 IP 相關路由
    ip::configure_routes(cfg);
}

async fn index() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "IP Lookup API",
        "version": "1.0.0",
        "endpoints": {
            "ipv4": "/api/v1/ip/v4",
            "ipv6": "/api/v1/ip/v6"
        }
    }))
}