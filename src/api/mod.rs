pub mod ip;
pub mod ddns;
pub mod cloudflare;
pub mod ddns_config;

use actix_web::{web, HttpResponse, Responder};

/// 配置所有 API 路由
/// 
/// # 端點
/// 
/// - GET /api/v1/: API 根端點
/// - GET /api/v1/ip/v4: 獲取 IPv4 地址
/// - GET /api/v1/ip/v6: 獲取 IPv6 地址
/// - GET /api/v1/ddns/update/ipv4: 更新 IPv4 DNS 記錄
/// - GET /api/v1/ddns/update/ipv6: 更新 IPv6 DNS 記錄
/// 
/// # 參數
/// 
/// - `cfg`: Web 服務配置
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/", web::get().to(index))
            .configure(ip::configure_routes)
            .configure(ddns::configure_routes)
            .configure(cloudflare::configure_routes)
            .configure(ddns_config::configure_routes)
    );
}

/// API 根端點處理函數
/// 
/// # 返回
/// 
/// - `impl Responder`: HTTP 響應
/// 
/// # 響應格式
/// 
/// ```json
/// {
///     "status": "success",
///     "message": "IP Lookup API",
///     "version": "1.0.0",
///     "endpoints": {
///         "ipv4": "/api/v1/ip/v4",
///         "ipv6": "/api/v1/ip/v6",
///         "ddns": {
///             "ipv4": "/api/v1/ddns/update/ipv4",
///             "ipv6": "/api/v1/ddns/update/ipv6"
///         },
///         "cloudflare": {
///             "zones": "/api/v1/cloudflare/zones",
///             "dns_records": "/api/v1/cloudflare/zones/{zone_id}/dns_records",
///             "create_record": "/api/v1/cloudflare/dns_records"
///         },
///         "ddns_config": {
///             "get": "/api/v1/ddns_config",
///             "save": "/api/v1/ddns_config/save",
///             "save_env": "/api/v1/ddns_config/save_env",
///             "delete": "/api/v1/ddns_config/delete",
///             "restart": "/api/v1/ddns_config/restart"
///         }
///     }
/// }
/// ```
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
            },
            "cloudflare": {
                "zones": "/api/v1/cloudflare/zones",
                "dns_records": "/api/v1/cloudflare/zones/{zone_id}/dns_records",
                "create_record": "/api/v1/cloudflare/dns_records"
            },
            "ddns_config": {
                "get": "/api/v1/ddns_config",
                "save": "/api/v1/ddns_config/save",
                "save_env": "/api/v1/ddns_config/save_env",
                "delete": "/api/v1/ddns_config/delete",
                "restart": "/api/v1/ddns_config/restart"
            }
        }
    }))
}