use actix_web::{web, HttpResponse, Responder, post};
use crate::application::ServiceFactory;
use serde::Serialize;
use log::info;
use std::sync::Arc;

/// 更新響應結構
#[derive(Serialize)]
pub struct UpdateResponse {
    success: bool,
    message: String,
    ip_address: Option<String>,
    domain: Option<String>,
}

/// 強制更新 DNS 記錄處理器
/// 
/// # 參數
/// 
/// - `service_factory`: 服務工廠
/// 
/// # 返回
/// 
/// - `impl Responder`: 返回更新結果
#[post("/update")]
pub async fn force_update(service_factory: web::Data<Arc<ServiceFactory>>) -> impl Responder {
    info!("收到強制更新DNS記錄請求");
    
    // 通過事件系統觸發強制更新
    let event_manager = service_factory.get_event_manager();
    event_manager.force_update_dns(None).await;
    
    let response = UpdateResponse {
        success: true,
        message: "DNS記錄更新請求已發送".to_string(),
        ip_address: None,
        domain: None,
    };
    
    HttpResponse::Ok().json(response)
}

/// 重啟 DDNS 服務處理器
/// 
/// # 參數
/// 
/// - `service_factory`: 服務工廠
/// 
/// # 返回
/// 
/// - `impl Responder`: 返回重啟結果
#[post("/restart")]
pub async fn restart_service(service_factory: web::Data<Arc<ServiceFactory>>) -> impl Responder {
    info!("收到重啟DDNS服務請求");
    
    // 通過事件系統觸發重啟
    let event_manager = service_factory.get_event_manager();
    event_manager.restart_ddns_service().await;
    
    let response = UpdateResponse {
        success: true,
        message: "DDNS服務重啟請求已發送".to_string(),
        ip_address: None,
        domain: None,
    };
    
    HttpResponse::Ok().json(response)
} 