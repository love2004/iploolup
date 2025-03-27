use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use log::{info, error};
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::env;
use std::collections::HashMap;
use crate::config::DdnsConfigLoader;
use crate::services::ddns::DdnsConfig;
use crate::error::AppError;
use std::process::Command;

// 用於API請求的認證模型
#[derive(Deserialize)]
pub struct ConfigRequest {
    action: String,
    config: Option<DdnsConfig>,
    config_name: Option<String>,
}

// DDNS環境變量配置結構
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvConfig {
    pub cloudflare_api_token: String,
    pub cloudflare_zone_id: String,
    pub cloudflare_record_id: String,
    pub cloudflare_record_name: String,
    pub update_interval: u64,
    pub ip_type: String,
}

// 配置清單響應
#[derive(Serialize)]
pub struct ConfigListResponse {
    pub configs: Vec<DdnsConfig>,
    pub env_configs: HashMap<String, String>,
}

// 全局的DDNS配置管理器
pub struct DdnsConfigManager {
    pub config_file: Mutex<Option<String>>,
}

impl DdnsConfigManager {
    pub fn new() -> Self {
        let config_file = match env::var("DDNS_CONFIG_FILE") {
            Ok(path) => Some(path),
            Err(_) => None,
        };
        
        Self {
            config_file: Mutex::new(config_file),
        }
    }
    
    // 獲取當前配置清單
    pub fn get_configs(&self) -> Result<ConfigListResponse, AppError> {
        // 獲取JSON配置
        let configs = match self.config_file.lock() {
            Ok(config_file) => {
                if let Some(file_path) = config_file.as_ref() {
                    if Path::new(file_path).exists() {
                        match fs::read_to_string(file_path) {
                            Ok(content) => {
                                match serde_json::from_str::<Vec<DdnsConfig>>(&content) {
                                    Ok(configs) => configs,
                                    Err(e) => {
                                        error!("Failed to parse config file: {}", e);
                                        Vec::new()
                                    }
                                }
                            },
                            Err(e) => {
                                error!("Failed to read config file: {}", e);
                                Vec::new()
                            }
                        }
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                }
            },
            Err(e) => {
                error!("Failed to lock config file: {}", e);
                Vec::new()
            }
        };
        
        // 獲取環境變量配置
        let mut env_configs = HashMap::new();
        let env_vars = [
            "CLOUDFLARE_API_TOKEN", "CLOUDFLARE_ZONE_ID", "CLOUDFLARE_RECORD_ID", 
            "CLOUDFLARE_RECORD_NAME", "DDNS_UPDATE_INTERVAL", "CLOUDFLARE_API_TOKEN_V6", 
            "CLOUDFLARE_ZONE_ID_V6", "CLOUDFLARE_RECORD_ID_V6", "CLOUDFLARE_RECORD_NAME_V6", 
            "DDNS_UPDATE_INTERVAL_V6"
        ];
        
        for var in env_vars.iter() {
            if let Ok(value) = env::var(var) {
                env_configs.insert(var.to_string(), value);
            }
        }
        
        Ok(ConfigListResponse { configs, env_configs })
    }
    
    // 添加新配置到配置文件
    pub fn add_config(&self, config: DdnsConfig) -> Result<(), AppError> {
        let mut configs = Vec::new();
        let mut file_path = String::new();
        
        // 獲取配置文件路徑和現有配置
        {
            let config_file_lock = self.config_file.lock()
                .map_err(|e| AppError::ConfigError(format!("Failed to lock config file: {}", e)))?;
            
            if let Some(path) = config_file_lock.as_ref() {
                file_path = path.clone();
                
                if Path::new(&file_path).exists() {
                    match fs::read_to_string(&file_path) {
                        Ok(content) => {
                            match serde_json::from_str::<Vec<DdnsConfig>>(&content) {
                                Ok(existing_configs) => {
                                    configs = existing_configs;
                                },
                                Err(e) => {
                                    return Err(AppError::ConfigError(format!("Failed to parse config file: {}", e)));
                                }
                            }
                        },
                        Err(e) => {
                            return Err(AppError::ConfigError(format!("Failed to read config file: {}", e)));
                        }
                    }
                }
            } else {
                // 如果沒有配置文件路徑，則創建一個默認的
                file_path = "config/ddns.json".to_string();
                
                // 更新配置文件路徑
                let mut config_file_mut = self.config_file.lock()
                    .map_err(|e| AppError::ConfigError(format!("Failed to lock config file: {}", e)))?;
                *config_file_mut = Some(file_path.clone());
                
                // 確保目錄存在
                if let Some(parent) = Path::new(&file_path).parent() {
                    if !parent.exists() {
                        match fs::create_dir_all(parent) {
                            Ok(_) => {},
                            Err(e) => {
                                return Err(AppError::ConfigError(format!("Failed to create config directory: {}", e)));
                            }
                        }
                    }
                }
                
                // 設置環境變量
                unsafe {
                    env::set_var("DDNS_CONFIG_FILE", &file_path);
                }
            }
        }
        
        // 檢查是否有相同IP類型和記錄名稱的配置
        let mut duplicate = false;
        let mut duplicate_idx = 0;
        
        for (i, existing) in configs.iter().enumerate() {
            if existing.ip_type == config.ip_type && existing.record_name == config.record_name {
                duplicate = true;
                duplicate_idx = i;
                break;
            }
        }
        
        // 更新或添加配置
        if duplicate {
            configs[duplicate_idx] = config;
        } else {
            configs.push(config);
        }
        
        // 寫入配置文件
        match serde_json::to_string_pretty(&configs) {
            Ok(json) => {
                match fs::write(&file_path, json) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(AppError::ConfigError(format!("Failed to write config file: {}", e)))
                }
            },
            Err(e) => Err(AppError::ConfigError(format!("Failed to serialize config: {}", e)))
        }
    }
    
    // 更新環境變量配置
    pub fn update_env_config(&self, ip_type: &str, config: EnvConfig) -> Result<(), AppError> {
        let prefix = if ip_type == "ipv6" { "_V6" } else { "" };
        
        unsafe {
            env::set_var(format!("CLOUDFLARE_API_TOKEN{}", prefix), &config.cloudflare_api_token);
            env::set_var(format!("CLOUDFLARE_ZONE_ID{}", prefix), &config.cloudflare_zone_id);
            env::set_var(format!("CLOUDFLARE_RECORD_ID{}", prefix), &config.cloudflare_record_id);
            env::set_var(format!("CLOUDFLARE_RECORD_NAME{}", prefix), &config.cloudflare_record_name);
            env::set_var(format!("DDNS_UPDATE_INTERVAL{}", prefix), config.update_interval.to_string());
        }
        
        // 嘗試保存到.env文件
        self.save_env_file()?;
        
        Ok(())
    }
    
    // 刪除配置
    pub fn delete_config(&self, config_name: &str, ip_type: &str) -> Result<(), AppError> {
        // 如果是環境變量配置
        if config_name == "env" {
            let prefix = if ip_type == "ipv6" { "_V6" } else { "" };
            
            unsafe {
                env::remove_var(format!("CLOUDFLARE_API_TOKEN{}", prefix));
                env::remove_var(format!("CLOUDFLARE_ZONE_ID{}", prefix));
                env::remove_var(format!("CLOUDFLARE_RECORD_ID{}", prefix));
                env::remove_var(format!("CLOUDFLARE_RECORD_NAME{}", prefix));
                env::remove_var(format!("DDNS_UPDATE_INTERVAL{}", prefix));
            }
            
            // 更新.env文件
            self.save_env_file()?;
            
            return Ok(());
        }
        
        // 如果是JSON配置
        let mut file_path = String::new();
        let mut configs = Vec::new();
        
        // 獲取配置文件路徑和現有配置
        {
            let config_file_lock = self.config_file.lock()
                .map_err(|e| AppError::ConfigError(format!("Failed to lock config file: {}", e)))?;
            
            if let Some(path) = config_file_lock.as_ref() {
                file_path = path.clone();
                
                if Path::new(&file_path).exists() {
                    match fs::read_to_string(&file_path) {
                        Ok(content) => {
                            match serde_json::from_str::<Vec<DdnsConfig>>(&content) {
                                Ok(existing_configs) => {
                                    configs = existing_configs;
                                },
                                Err(e) => {
                                    return Err(AppError::ConfigError(format!("Failed to parse config file: {}", e)));
                                }
                            }
                        },
                        Err(e) => {
                            return Err(AppError::ConfigError(format!("Failed to read config file: {}", e)));
                        }
                    }
                } else {
                    return Err(AppError::ConfigError("Config file does not exist".to_string()));
                }
            } else {
                return Err(AppError::ConfigError("No config file configured".to_string()));
            }
        }
        
        // 查找要刪除的配置
        let mut found = false;
        configs.retain(|c| {
            if c.record_name == config_name && c.ip_type == ip_type {
                found = true;
                false
            } else {
                true
            }
        });
        
        if !found {
            return Err(AppError::ConfigError(format!("Config with name {} and type {} not found", config_name, ip_type)));
        }
        
        // 寫入配置文件
        match serde_json::to_string_pretty(&configs) {
            Ok(json) => {
                match fs::write(&file_path, json) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(AppError::ConfigError(format!("Failed to write config file: {}", e)))
                }
            },
            Err(e) => Err(AppError::ConfigError(format!("Failed to serialize config: {}", e)))
        }
    }
    
    // 保存環境變量到.env文件
    fn save_env_file(&self) -> Result<(), AppError> {
        let env_file = ".env";
        let env_vars = [
            "CLOUDFLARE_API_TOKEN", "CLOUDFLARE_ZONE_ID", "CLOUDFLARE_RECORD_ID", 
            "CLOUDFLARE_RECORD_NAME", "DDNS_UPDATE_INTERVAL", "CLOUDFLARE_API_TOKEN_V6", 
            "CLOUDFLARE_ZONE_ID_V6", "CLOUDFLARE_RECORD_ID_V6", "CLOUDFLARE_RECORD_NAME_V6", 
            "DDNS_UPDATE_INTERVAL_V6", "DDNS_CONFIG_FILE"
        ];
        
        let mut content = String::new();
        
        for var in env_vars.iter() {
            if let Ok(value) = env::var(var) {
                content.push_str(&format!("{}={}\n", var, value));
            }
        }
        
        match fs::write(env_file, content) {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::ConfigError(format!("Failed to write .env file: {}", e)))
        }
    }
}

// 獲取DDNS配置
pub async fn get_ddns_configs(
    config_manager: web::Data<DdnsConfigManager>
) -> impl Responder {
    match config_manager.get_configs() {
        Ok(config_list) => {
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "configs": config_list.configs,
                "env_configs": config_list.env_configs
            }))
        },
        Err(e) => {
            error!("Failed to get DDNS configs: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to get DDNS configs: {}", e)
            }))
        }
    }
}

// 保存DDNS配置
pub async fn save_ddns_config(
    config_manager: web::Data<DdnsConfigManager>,
    config: web::Json<DdnsConfig>
) -> impl Responder {
    match config_manager.add_config(config.into_inner()) {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "message": "DDNS configuration saved successfully"
            }))
        },
        Err(e) => {
            error!("Failed to save DDNS config: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to save DDNS config: {}", e)
            }))
        }
    }
}

// 保存DDNS環境變量配置
pub async fn save_env_config(
    config_manager: web::Data<DdnsConfigManager>,
    config_data: web::Json<(String, EnvConfig)>
) -> impl Responder {
    let (ip_type, config) = config_data.into_inner();
    
    match config_manager.update_env_config(&ip_type, config) {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "message": format!("{} environment configuration saved successfully", ip_type)
            }))
        },
        Err(e) => {
            error!("Failed to save environment config: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to save environment config: {}", e)
            }))
        }
    }
}

// 刪除DDNS配置
pub async fn delete_ddns_config(
    config_manager: web::Data<DdnsConfigManager>,
    config_data: web::Json<(String, String)>
) -> impl Responder {
    let (config_name, ip_type) = config_data.into_inner();
    
    match config_manager.delete_config(&config_name, &ip_type) {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "message": format!("DDNS configuration for {} ({}) deleted successfully", config_name, ip_type)
            }))
        },
        Err(e) => {
            error!("Failed to delete DDNS config: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to delete DDNS config: {}", e)
            }))
        }
    }
}

// 重啟DDNS服務
pub async fn restart_ddns_service() -> impl Responder {
    info!("正在重啟DDNS服務...");
    
    // 獲取當前可執行文件路徑
    if let Ok(current_exe) = env::current_exe() {
        // 啟動一個新的DDNS進程
        match Command::new(&current_exe)
            .env("RUST_LOG", env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
            .env("RUN_MODE", "ddns")
            .spawn() {
                Ok(child) => {
                    info!("DDNS服務已在PID: {}啟動", child.id());
                    
                    HttpResponse::Ok().json(serde_json::json!({
                        "status": "success",
                        "message": format!("DDNS服務已重新啟動，新進程PID: {}", child.id())
                    }))
                }
                Err(e) => {
                    error!("無法啟動DDNS服務: {}", e);
                    
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "status": "error",
                        "message": format!("無法啟動DDNS服務: {}", e)
                    }))
                }
            }
    } else {
        error!("無法獲取當前可執行文件路徑");
        
        HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "無法獲取當前可執行文件路徑"
        }))
    }
} 