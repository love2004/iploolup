pub mod ip;
pub mod ddns;

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
            }
        }
    }))
}