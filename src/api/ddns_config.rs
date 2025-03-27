use actix_web::{web, Responder};
use crate::services::ddns_config::{get_ddns_configs, save_ddns_config, save_env_config, delete_ddns_config, restart_ddns_service};

/// 配置DDNS配置API路由
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ddns_config")
            .route("", web::get().to(get_ddns_configs))
            .route("/save", web::post().to(save_ddns_config))
            .route("/save_env", web::post().to(save_env_config))
            .route("/delete", web::post().to(delete_ddns_config))
            .route("/restart", web::post().to(restart_ddns_service))
    );
} 