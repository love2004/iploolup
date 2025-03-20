use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;

/// 伺服器設置結構
/// 
/// # 欄位
/// 
/// - `host`: 伺服器監聽的主機地址
/// - `port`: 伺服器監聽的端口
#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

/// 應用程式設置結構
/// 
/// # 欄位
/// 
/// - `server`: 伺服器相關設置
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
}

impl Settings {
    /// 創建新的設置實例
    /// 
    /// # 功能
    /// 
    /// 從配置文件加載設置
    /// 
    /// # 返回
    /// 
    /// - `Result<Self, ConfigError>`: 成功時返回設置實例，失敗時返回錯誤
    /// 
    /// # 配置文件
    /// 
    /// - `config/default.toml`: 默認設置
    /// - `config/{run_mode}.toml`: 環境特定設置
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        
        let s = Config::builder()
            .add_source(File::with_name("config/default").required(true))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .build()?;
            
        s.try_deserialize()
    }
}