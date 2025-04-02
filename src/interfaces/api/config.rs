use actix_web::{web, get, post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::domain::config::DdnsConfig;
use crate::ServiceFactory;
use log::{info, error};

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
/// - `GET /api/config`
#[get("/config")]
pub async fn get_configs(
    service_factory: web::Data<std::sync::Arc<ServiceFactory>>
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
            error!("獲取配置失敗: {}", e);
            HttpResponse::InternalServerError().json(ConfigResponse {
                success: false,
                message: format!("獲取配置失敗: {}", e),
                configs: None,
            })
        }
    }
}

/// 保存配置
/// 
/// # 路由
/// 
/// - `POST /api/config`
/// 
/// # 請求體
/// 
/// - 要保存的配置
/// 
/// # 返回
/// 
/// - 配置保存結果
#[post("/config")]
pub async fn save_configs(
    service_factory: web::Data<std::sync::Arc<ServiceFactory>>,
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
            error!("保存配置失敗: {}", e);
            HttpResponse::InternalServerError().json(ConfigResponse {
                success: false,
                message: format!("保存配置失敗: {}", e),
                configs: None,
            })
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
#[post("/config/validate")]
pub async fn validate_config(
    req: web::Json<ValidateConfigRequest>
) -> impl Responder {
    info!("收到驗證配置請求: {}", req.config.record_name);
    
    // 驗證配置
    match req.config.validate() {
        Ok(_) => {
            HttpResponse::Ok().json(ValidateConfigResponse {
                success: true,
                message: "配置驗證通過".to_string(),
                is_valid: true,
            })
        },
        Err(e) => {
            info!("配置驗證失敗: {}", e);
            HttpResponse::Ok().json(ValidateConfigResponse {
                success: true,
                message: format!("配置驗證失敗: {}", e),
                is_valid: false,
            })
        }
    }
} 