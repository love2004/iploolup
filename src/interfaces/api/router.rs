use actix_web::web;
use super::ip::{get_ipv4, get_ipv6};
use super::health::health_check;
use super::status::get_status;
use super::update::{force_update, restart_service};
use super::config::{get_configs, save_configs, validate_config};

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
            .service(
                web::scope("/ip")
                    .service(get_ipv4)
                    .service(get_ipv6)
            )
            .service(health_check)
            .service(get_status)
            .service(force_update)
            .service(restart_service)
            .service(get_configs)
            .service(save_configs)
            .service(validate_config)
    );
} 