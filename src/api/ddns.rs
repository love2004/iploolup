use actix_web::{web, HttpResponse, Responder};
use crate::error::AppError;
use crate::services::ddns::DdnsService;
use crate::config::DdnsConfigLoader;

/// 配置 DDNS 相關的路由
/// 
/// # 端點
/// 
/// - GET /ddns/update/ipv4: 更新 IPv4 DNS 記錄
/// - GET /ddns/update/ipv6: 更新 IPv6 DNS 記錄
/// - GET /ddns/update: IPv4 更新的向下兼容端點
/// 
/// # 參數
/// 
/// - `cfg`: Web 服務配置
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ddns")
            .route("/update/ipv4", web::get().to(update_ipv4_record))
            .route("/update/ipv6", web::get().to(update_ipv6_record))
            .route("/update", web::get().to(update_ipv4_record)) // 向下兼容
    );
}

/// 更新 IPv4 DNS 記錄的處理函數
/// 
/// # 功能
/// 
/// 載入 IPv4 DDNS 配置，並使用 Cloudflare API 更新 IPv4 DNS 記錄
/// 
/// # 返回
/// 
/// - `Result<impl Responder, AppError>`: 成功時返回 HTTP 響應，失敗時返回錯誤
/// 
/// # 錯誤
/// 
/// 當以下情況發生時返回錯誤：
/// - 配置讀取失敗
/// - DNS 更新失敗
/// - API 請求失敗
async fn update_ipv4_record() -> Result<impl Responder, AppError> {
    let config = DdnsConfigLoader::load_for_api("ipv4")?;
    let ddns_service = DdnsService::new(config);
    let result = ddns_service.update_record().await?;
    
    Ok(HttpResponse::Ok().json(result))
}

/// 更新 IPv6 DNS 記錄的處理函數
/// 
/// # 功能
/// 
/// 載入 IPv6 DDNS 配置，並使用 Cloudflare API 更新 IPv6 DNS 記錄
/// 
/// # 返回
/// 
/// - `Result<impl Responder, AppError>`: 成功時返回 HTTP 響應，失敗時返回錯誤
/// 
/// # 錯誤
/// 
/// 當以下情況發生時返回錯誤：
/// - 配置讀取失敗
/// - DNS 更新失敗
/// - API 請求失敗
async fn update_ipv6_record() -> Result<impl Responder, AppError> {
    let config = DdnsConfigLoader::load_for_api("ipv6")?;
    let ddns_service = DdnsService::new(config);
    let result = ddns_service.update_record().await?;
    
    Ok(HttpResponse::Ok().json(result))
} 