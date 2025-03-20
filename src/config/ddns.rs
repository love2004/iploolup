use std::env;
use std::fs;
use serde_json;
use crate::error::AppError;
use crate::services::ddns::DdnsConfig;
use log::{info, warn, error};

/// DDNS 配置加載器
/// 
/// 提供統一的配置加載邏輯，支持從環境變量或配置文件加載
pub struct DdnsConfigLoader;

impl DdnsConfigLoader {
    /// 從環境變量或配置文件加載所有 DDNS 配置
    /// 
    /// # 返回
    /// 
    /// - `Result<Vec<DdnsConfig>, AppError>`: 成功時返回 DDNS 配置列表，失敗時返回錯誤
    pub fn load_all_configs() -> Result<Vec<DdnsConfig>, AppError> {
        let mut configs = Vec::new();
        
        // 首先檢查是否有配置文件
        if let Ok(config_file) = env::var("DDNS_CONFIG_FILE") {
            info!("Loading DDNS settings from config file: {}", config_file);
            
            match Self::load_from_file(&config_file) {
                Ok(file_configs) => {
                    if file_configs.is_empty() {
                        warn!("No valid DDNS configurations in config file, will try using environment variables");
                    } else {
                        info!("Successfully loaded {} DDNS configurations from config file", file_configs.len());
                        return Ok(file_configs);
                    }
                }
                Err(e) => {
                    error!("Failed to load config file: {}, will try using environment variables", e);
                }
            }
        }
        
        // 如果沒有配置文件或者讀取失敗，則使用環境變量
        info!("Loading DDNS settings from environment variables");
        
        // 嘗試加載 IPv4 配置
        match Self::load_ipv4_config() {
            Ok(config) => {
                info!("IPv4 DDNS service configured");
                configs.push(config);
            }
            Err(e) => {
                warn!("Unable to configure IPv4 DDNS service: {}", e);
            }
        }
        
        // 嘗試加載 IPv6 配置
        match Self::load_ipv6_config() {
            Ok(config) => {
                info!("IPv6 DDNS service configured");
                configs.push(config);
            }
            Err(e) => {
                warn!("Unable to configure IPv6 DDNS service: {}", e);
            }
        }
        
        if configs.is_empty() {
            return Err(AppError::ConfigError("Unable to load any DDNS configurations".to_string()));
        }
        
        Ok(configs)
    }
    
    /// 加載用於 API 的 DDNS 配置
    /// 
    /// # 參數
    /// 
    /// - `ip_type`: IP 類型（"ipv4" 或 "ipv6"）
    /// 
    /// # 返回
    /// 
    /// - `Result<DdnsConfig, AppError>`: 成功時返回 DDNS 配置，失敗時返回錯誤
    pub fn load_for_api(ip_type: &str) -> Result<DdnsConfig, AppError> {
        match ip_type {
            "ipv4" => Self::load_ipv4_config(),
            "ipv6" => Self::load_ipv6_config(),
            _ => Err(AppError::ConfigError(format!("Invalid IP type: {}", ip_type)))
        }
    }
    
    /// 從配置文件加載 DDNS 配置
    /// 
    /// # 參數
    /// 
    /// - `file_path`: 配置文件路徑
    /// 
    /// # 返回
    /// 
    /// - `Result<Vec<DdnsConfig>, AppError>`: 成功時返回 DDNS 配置列表，失敗時返回錯誤
    fn load_from_file(file_path: &str) -> Result<Vec<DdnsConfig>, AppError> {
        let file_content = fs::read_to_string(file_path)
            .map_err(|e| AppError::ConfigError(format!("Failed to parse configuration file: {}", e)))?;
            
        let configs: Vec<DdnsConfig> = serde_json::from_str(&file_content)
            .map_err(|e| AppError::ConfigError(format!("Failed to parse configuration file: {}", e)))?;
            
        // 驗證配置
        for (i, config) in configs.iter().enumerate() {
            Self::validate_config(config, &format!("Configuration[{}]", i))?;
        }
        
        Ok(configs)
    }
    
    /// 加載 IPv4 DDNS 配置
    /// 
    /// # 返回
    /// 
    /// - `Result<DdnsConfig, AppError>`: 成功時返回 DDNS 配置，失敗時返回錯誤
    fn load_ipv4_config() -> Result<DdnsConfig, AppError> {
        let api_token = env::var("CLOUDFLARE_API_TOKEN")
            .map_err(|_| AppError::ConfigError("Missing CLOUDFLARE_API_TOKEN environment variable".to_string()))?;
        
        let zone_id = env::var("CLOUDFLARE_ZONE_ID")
            .map_err(|_| AppError::ConfigError("Missing CLOUDFLARE_ZONE_ID environment variable".to_string()))?;
        
        let record_id = env::var("CLOUDFLARE_RECORD_ID")
            .map_err(|_| AppError::ConfigError("Missing CLOUDFLARE_RECORD_ID environment variable".to_string()))?;
        
        let record_name = env::var("CLOUDFLARE_RECORD_NAME")
            .map_err(|_| AppError::ConfigError("Missing CLOUDFLARE_RECORD_NAME environment variable".to_string()))?;
        
        let update_interval = env::var("DDNS_UPDATE_INTERVAL")
            .unwrap_or_else(|_| "300".to_string())
            .parse()
            .map_err(|_| AppError::ConfigError("DDNS_UPDATE_INTERVAL must be a number".to_string()))?;
        
        let config = DdnsConfig {
            api_token,
            zone_id,
            record_id,
            record_name,
            update_interval,
            ip_type: "ipv4".to_string(),
        };
        
        Self::validate_config(&config, "IPv4 Configuration")?;
        
        Ok(config)
    }
    
    /// 加載 IPv6 DDNS 配置
    /// 
    /// # 返回
    /// 
    /// - `Result<DdnsConfig, AppError>`: 成功時返回 DDNS 配置，失敗時返回錯誤
    fn load_ipv6_config() -> Result<DdnsConfig, AppError> {
        // 優先使用專用的 IPv6 API 令牌和區域 ID
        let api_token = if let Ok(token) = env::var("CLOUDFLARE_API_TOKEN_V6") {
            token
        } else {
            env::var("CLOUDFLARE_API_TOKEN")
                .map_err(|_| AppError::ConfigError("Missing API token environment variable".to_string()))?
        };
        
        let zone_id = if let Ok(zone) = env::var("CLOUDFLARE_ZONE_ID_V6") {
            zone
        } else {
            env::var("CLOUDFLARE_ZONE_ID")
                .map_err(|_| AppError::ConfigError("Missing zone ID environment variable".to_string()))?
        };
        
        // IPv6 記錄 ID 和名稱是必需的
        let record_id = env::var("CLOUDFLARE_RECORD_ID_V6")
            .map_err(|_| AppError::ConfigError("Missing CLOUDFLARE_RECORD_ID_V6 environment variable".to_string()))?;
        
        let record_name = env::var("CLOUDFLARE_RECORD_NAME_V6")
            .map_err(|_| AppError::ConfigError("Missing CLOUDFLARE_RECORD_NAME_V6 environment variable".to_string()))?;
        
        // 可以使用專用更新間隔或與 IPv4 相同的間隔
        let update_interval = env::var("DDNS_UPDATE_INTERVAL_V6")
            .unwrap_or_else(|_| 
                env::var("DDNS_UPDATE_INTERVAL").unwrap_or_else(|_| "300".to_string())
            )
            .parse()
            .map_err(|_| AppError::ConfigError("Update interval must be a number".to_string()))?;
        
        let config = DdnsConfig {
            api_token,
            zone_id,
            record_id,
            record_name,
            update_interval,
            ip_type: "ipv6".to_string(),
        };
        
        Self::validate_config(&config, "IPv6 Configuration")?;
        
        Ok(config)
    }
    
    /// 驗證 DDNS 配置
    /// 
    /// # 參數
    /// 
    /// - `config`: 要驗證的配置
    /// - `context`: 錯誤上下文描述
    /// 
    /// # 返回
    /// 
    /// - `Result<(), AppError>`: 成功時返回 ()，失敗時返回錯誤
    fn validate_config(config: &DdnsConfig, context: &str) -> Result<(), AppError> {
        // 驗證 API 令牌
        if config.api_token.trim().is_empty() {
            return Err(AppError::ConfigError(format!("{}: API token cannot be empty", context)));
        }
        
        // 驗證區域 ID
        if config.zone_id.trim().is_empty() {
            return Err(AppError::ConfigError(format!("{}: Zone ID cannot be empty", context)));
        }
        
        // 驗證記錄 ID
        if config.record_id.trim().is_empty() {
            return Err(AppError::ConfigError(format!("{}: Record ID cannot be empty", context)));
        }
        
        // 驗證記錄名稱
        if config.record_name.trim().is_empty() {
            return Err(AppError::ConfigError(format!("{}: Record name cannot be empty", context)));
        }
        
        // 驗證更新間隔
        if config.update_interval < 5 {
            return Err(AppError::ConfigError(format!("{}: Update interval cannot be less than 5 seconds", context)));
        }
        
        // 驗證 IP 類型
        match config.ip_type.as_str() {
            "ipv4" | "ipv6" => Ok(()),
            _ => Err(AppError::ConfigError(format!("{}: IP type must be ipv4 or ipv6", context)))
        }
    }
} 