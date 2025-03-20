use actix_web::{web, HttpResponse, Responder};
use crate::services::ip;

/// 配置 IP 相關的路由
/// 
/// # 端點
/// 
/// - GET /ip/v4: 獲取當前 IPv4 地址
/// - GET /ip/v6: 獲取當前 IPv6 地址
/// 
/// # 參數
/// 
/// - `cfg`: Web 服務配置
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ip")
            .route("/v4", web::get().to(get_ipv4))
            .route("/v6", web::get().to(get_ipv6))
    );
}

/// 獲取當前 IPv4 地址的處理函數
/// 
/// # 返回
/// 
/// - `impl Responder`: HTTP 響應
/// 
/// # 響應格式
/// 
/// 成功時：
/// ```json
/// {
///     "status": "success",
///     "data": {
///         "ip": "xxx.xxx.xxx.xxx"
///     }
/// }
/// ```
/// 
/// 失敗時：
/// ```json
/// {
///     "status": "error",
///     "message": "錯誤訊息"
/// }
/// ```
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

/// 獲取當前 IPv6 地址的處理函數
/// 
/// # 返回
/// 
/// - `impl Responder`: HTTP 響應
/// 
/// # 響應格式
/// 
/// 成功時：
/// ```json
/// {
///     "status": "success",
///     "data": {
///         "ip": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx"
///     }
/// }
/// ```
/// 
/// 失敗時：
/// ```json
/// {
///     "status": "error",
///     "message": "錯誤訊息"
/// }
/// ```
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