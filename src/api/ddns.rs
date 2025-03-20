use actix_web::{web, HttpResponse, Responder};
use std::env;
use crate::error::AppError;
use crate::services::ddns::{DdnsConfig, DdnsService};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ddns")
            .route("/update/ipv4", web::get().to(update_ipv4_record))
            .route("/update/ipv6", web::get().to(update_ipv6_record))
            .route("/update", web::get().to(update_ipv4_record)) // 向下兼容
    );
}

// 獲取 Cloudflare 配置
fn get_cloudflare_config() -> Result<DdnsConfig, AppError> {
    let config = DdnsConfig {
        api_token: env::var("CLOUDFLARE_API_TOKEN")
            .map_err(|_| AppError::ConfigError("缺少 CLOUDFLARE_API_TOKEN 環境變數".to_string()))?,
        zone_id: env::var("CLOUDFLARE_ZONE_ID")
            .map_err(|_| AppError::ConfigError("缺少 CLOUDFLARE_ZONE_ID 環境變數".to_string()))?,
        record_id: env::var("CLOUDFLARE_RECORD_ID")
            .map_err(|_| AppError::ConfigError("缺少 CLOUDFLARE_RECORD_ID 環境變數".to_string()))?,
        record_name: env::var("CLOUDFLARE_RECORD_NAME")
            .map_err(|_| AppError::ConfigError("缺少 CLOUDFLARE_RECORD_NAME 環境變數".to_string()))?,
    };
    
    Ok(config)
}

async fn update_ipv4_record() -> Result<impl Responder, AppError> {
    let config = get_cloudflare_config()?;
    let ddns_service = DdnsService::new(config);
    let result = ddns_service.update_record().await?;
    
    Ok(HttpResponse::Ok().json(result))
}

async fn update_ipv6_record() -> Result<impl Responder, AppError> {
    let config = get_cloudflare_config()?;
    let ddns_service = DdnsService::new(config);
    let result = ddns_service.update_ipv6_record().await?;
    
    Ok(HttpResponse::Ok().json(result))
} 