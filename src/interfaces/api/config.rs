use actix_web::{web, get, post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::domain::config::DdnsConfig;
use crate::application::ServiceFactory;
use crate::application::error::ApplicationError;
use log::{info, error};
use std::sync::Arc;
use super::common::handle_application_error;

/// 配置響應
#[derive(Serialize)]
struct ConfigResponse {
    success: bool,
    message: String,
    configs: Option<Vec<DdnsConfig>>,
}

/// 配置保存請求
#[derive(Deserialize)]
pub struct SaveConfigRequest {
    configs: Vec<DdnsConfig>,
}

/// 配置驗證請求
#[derive(Deserialize)]
pub struct ValidateConfigRequest {
    config: DdnsConfig,
}

/// 配置驗證響應
#[derive(Serialize)]
struct ValidateConfigResponse {
    success: bool,
    message: String,
    is_valid: bool,
}

/// 獲取當前配置
/// 
/// # 路由
/// 
/// - `GET /api/configs`
#[get("")]
pub async fn get_configs(
    service_factory: web::Data<Arc<ServiceFactory>>
) -> impl Responder {
    info!("收到獲取配置請求");
    
    let config_service = service_factory.get_config_service();
    
    match config_service.get_configs().await {
        Ok(configs) => {
            HttpResponse::Ok().json(ConfigResponse {
                success: true,
                message: format!("成功獲取 {} 個配置", configs.len()),
                configs: Some(configs),
            })
        },
        Err(e) => {
            let app_error: ApplicationError = ApplicationError::DomainError(e);
            handle_application_error(app_error, "獲取配置失敗")
        }
    }
}

/// 保存配置
/// 
/// # 路由
/// 
/// - `POST /api/configs`
/// 
/// # 請求體
/// 
/// - 要保存的配置
/// 
/// # 返回
/// 
/// - 配置保存結果
#[post("")]
pub async fn save_configs(
    service_factory: web::Data<Arc<ServiceFactory>>,
    req: web::Json<SaveConfigRequest>
) -> impl Responder {
    info!("收到保存配置請求，共 {} 個配置", req.configs.len());
    
    let result = service_factory.save_configs_and_apply(req.configs.clone()).await;
    
    match result {
        Ok(_) => {
            HttpResponse::Ok().json(ConfigResponse {
                success: true,
                message: format!("成功保存 {} 個配置", req.configs.len()),
                configs: Some(req.configs.clone()),
            })
        },
        Err(e) => {
            let app_error: ApplicationError = ApplicationError::DomainError(e);
            handle_application_error(app_error, "保存配置失敗")
        }
    }
}

/// 驗證配置
/// 
/// # 路由
/// 
/// - `POST /api/config/validate`
/// 
/// # 請求體
/// 
/// - 要驗證的配置
/// 
/// # 返回
/// 
/// - 配置驗證結果
#[post("/validate")]
pub async fn validate_config(
    req: web::Json<ValidateConfigRequest>
) -> impl Responder {
    info!("收到驗證配置請求: {}", req.config.record_name);
    
    // 驗證配置
    match req.config.validate() {
        Ok(_) => {
            info!("配置驗證通過: {}", req.config.record_name);
            HttpResponse::Ok().json(ValidateConfigResponse {
                success: true,
                message: "配置驗證通過".to_string(),
                is_valid: true,
            })
        },
        Err(e) => {
            // 驗證錯誤是預期的情況，不使用錯誤處理函數
            error!("配置驗證失敗: {}", e);
            HttpResponse::Ok().json(ValidateConfigResponse {
                success: false,
                message: format!("配置驗證失敗: {}", e),
                is_valid: false,
            })
        }
    }
} 