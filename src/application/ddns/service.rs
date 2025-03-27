use crate::domain::config::DdnsConfig;
use crate::domain::dns::{DnsRecord, DnsService, DnsUpdateResult};
use crate::domain::error::DomainError;
use crate::domain::ip::IpService;
use crate::domain::state::StateRepository;
use std::sync::Arc;
use chrono::Utc;
use log::{info, error, debug};
use std::time::Duration as StdDuration;
use tokio::time::sleep;

/// DDNS 應用服務
pub struct DdnsApplicationService {
    dns_service: Arc<dyn DnsService>,
    ip_service: Arc<dyn IpService>,
    state_repository: Arc<dyn StateRepository>,
    config: DdnsConfig,
}

impl DdnsApplicationService {
    /// 創建新的 DDNS 應用服務
    ///
    /// # 參數
    ///
    /// - `dns_service`: DNS 服務
    /// - `ip_service`: IP 服務
    /// - `state_repository`: 狀態存儲庫
    /// - `config`: DDNS 配置
    pub fn new(
        dns_service: Arc<dyn DnsService>,
        ip_service: Arc<dyn IpService>,
        state_repository: Arc<dyn StateRepository>,
        config: DdnsConfig,
    ) -> Self {
        Self {
            dns_service,
            ip_service,
            state_repository,
            config,
        }
    }
    
    /// 獲取當前 IP 地址
    ///
    /// # 返回
    ///
    /// - `Result<String, DomainError>`: 成功時返回 IP 地址，失敗時返回錯誤
    async fn get_current_ip(&self) -> Result<String, DomainError> {
        match self.config.ip_type.to_string().as_str() {
            "ipv4" => self.ip_service.get_ipv4().await,
            "ipv6" => self.ip_service.get_ipv6().await,
            _ => Err(DomainError::ValidationError(format!("Invalid IP type: {}", self.config.ip_type))),
        }
    }
    
    /// 檢查 IP 是否變更
    ///
    /// # 參數
    ///
    /// - `current_ip`: 當前 IP 地址
    ///
    /// # 返回
    ///
    /// - `Result<bool, DomainError>`: 成功時返回是否變更，失敗時返回錯誤
    async fn is_ip_changed(&self, current_ip: &str) -> Result<bool, DomainError> {
        let config_id = format!("{}-{}", self.config.zone_id, self.config.record_id);
        let last_ip = self.state_repository.get_last_ip(&config_id).await?;
        
        match last_ip {
            Some(ip) => Ok(ip != current_ip),
            None => Ok(true), // 沒有記錄，視為變更
        }
    }
    
    /// 更新 DNS 記錄
    ///
    /// # 返回
    ///
    /// - `Result<DnsUpdateResult, DomainError>`: 成功時返回更新結果，失敗時返回錯誤
    pub async fn update_dns_record(&self) -> Result<DnsUpdateResult, DomainError> {
        // 獲取當前 IP
        let current_ip = self.get_current_ip().await?;
        debug!("Current {} address: {}", self.config.ip_type, current_ip);
        
        // 檢查 IP 是否變更
        let config_id = format!("{}-{}", self.config.zone_id, self.config.record_id);
        let is_changed = self.is_ip_changed(&current_ip).await?;
        
        if !is_changed {
            debug!("IP has not changed, skipping DNS update");
            
            // 創建一個更新結果，但標記為未實際更新
            let record = DnsRecord {
                id: Some(self.config.record_id.clone()),
                name: self.config.record_name.clone(),
                record_type: match self.config.ip_type.to_string().as_str() {
                    "ipv4" => "A".to_string(),
                    "ipv6" => "AAAA".to_string(),
                    _ => return Err(DomainError::ValidationError("Invalid IP type".to_string())),
                },
                content: current_ip,
                ttl: 120,
                proxied: false,
            };
            
            return Ok(DnsUpdateResult {
                record,
                updated: false,
            });
        }
        
        // 創建 DNS 記錄對象
        let record = DnsRecord {
            id: Some(self.config.record_id.clone()),
            name: self.config.record_name.clone(),
            record_type: match self.config.ip_type.to_string().as_str() {
                "ipv4" => "A".to_string(),
                "ipv6" => "AAAA".to_string(),
                _ => return Err(DomainError::ValidationError("Invalid IP type".to_string())),
            },
            content: current_ip.clone(),
            ttl: 120,
            proxied: false,
        };
        
        info!("Updating {} DNS record: {} to {}", self.config.ip_type, self.config.record_name, current_ip);
        
        // 更新 DNS 記錄
        let result = self.dns_service.update_record(record).await?;
        
        // 更新狀態
        if result.updated {
            self.state_repository.set_last_ip(&config_id, &current_ip).await?;
            self.state_repository.set_last_update_time(&config_id, Utc::now()).await?;
        }
        
        Ok(result)
    }
    
    /// 啟動自動更新服務
    ///
    /// # 功能
    ///
    /// 按照配置的間隔定期檢查 IP 並更新 DNS 記錄
    pub async fn start_auto_update(&self) {
        let interval = StdDuration::from_secs(self.config.update_interval);
        
        info!("Starting {} DDNS auto-update service for {}, update interval: {} seconds", 
              self.config.ip_type, self.config.record_name, self.config.update_interval);
        
        loop {
            match self.update_dns_record().await {
                Ok(result) => {
                    if result.updated {
                        info!("Successfully updated {} DNS record for {} to {}", 
                             self.config.ip_type, self.config.record_name, result.record.content);
                    } else {
                        debug!("No update needed for {} DNS record {}", 
                              self.config.ip_type, self.config.record_name);
                    }
                },
                Err(e) => {
                    error!("Failed to update {} DNS record: {}", self.config.ip_type, e);
                }
            }
            
            // 等待下一次更新
            debug!("Waiting {} seconds for next update", self.config.update_interval);
            sleep(interval).await;
        }
    }
} 