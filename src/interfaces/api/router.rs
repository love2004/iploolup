use actix_web::web;
use super::ip::{get_ipv4, get_ipv6};
use super::health::health_check;

/// 配置 API 路由
/// 
/// # 參數
/// 
/// - `cfg`: 服務配置
/// 
/// # 功能
/// 
/// - 註冊 API 路由
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_ipv4)
            .service(get_ipv6)
            .service(health_check)
    );
} 