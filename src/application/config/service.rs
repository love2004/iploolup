use crate::domain::config::DdnsConfig;
use crate::domain::error::DomainError;

/// 配置服務
pub struct ConfigService {
    // 後續可添加配置存儲庫等依賴
}

impl Default for ConfigService {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigService {
    /// 創建新的配置服務
    pub fn new() -> Self {
        Self {}
    }

    /// 載入所有配置
    pub fn load_configs(&self) -> Result<Vec<DdnsConfig>, DomainError> {
        // 後續將從配置存儲庫載入
        Ok(Vec::new())
    }

    /// 保存配置
    pub fn save_config(&self, _config: DdnsConfig) -> Result<(), DomainError> {
        // 後續將保存到配置存儲庫
        Ok(())
    }
} 