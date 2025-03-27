use crate::domain::config::DdnsConfig;
use crate::domain::dns::DnsService;
use crate::domain::ip::IpService;
use crate::domain::state::StateRepository;
use crate::infrastructure::http::ReqwestHttpClient;
use crate::infrastructure::ip::PublicIpService;
use crate::infrastructure::dns::CloudflareDnsService;
use crate::infrastructure::state::InMemoryStateRepository;
use crate::application::ddns::DdnsApplicationService;
use std::sync::Arc;

/// 服務工廠，用於創建和組裝服務
pub struct ServiceFactory {
    http_client: Arc<ReqwestHttpClient>,
    ip_service: Arc<dyn IpService>,
    state_repository: Arc<dyn StateRepository>,
}

impl Default for ServiceFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceFactory {
    /// 創建新的服務工廠
    pub fn new() -> Self {
        let http_client = Arc::new(ReqwestHttpClient::new());
        let ip_service = Arc::new(PublicIpService::new(
            http_client.clone(),
            None,
            None,
        ));
        let state_repository = Arc::new(InMemoryStateRepository::new());
        
        Self {
            http_client,
            ip_service,
            state_repository,
        }
    }
    
    /// 創建 DNS 服務
    ///
    /// # 參數
    ///
    /// - `config`: DDNS 配置
    ///
    /// # 返回
    ///
    /// - 實現 DnsService 接口的服務實例
    pub fn create_dns_service(&self, config: &DdnsConfig) -> Arc<dyn DnsService> {
        Arc::new(CloudflareDnsService::new(
            self.http_client.clone(),
            config.clone(),
        ))
    }
    
    /// 創建 DDNS 應用服務
    ///
    /// # 參數
    ///
    /// - `config`: DDNS 配置
    ///
    /// # 返回
    ///
    /// - DDNS 應用服務實例
    pub fn create_ddns_service(&self, config: DdnsConfig) -> DdnsApplicationService {
        let dns_service = self.create_dns_service(&config);
        
        DdnsApplicationService::new(
            dns_service,
            self.ip_service.clone(),
            self.state_repository.clone(),
            config,
        )
    }
    
    /// 獲取 IP 服務
    pub fn get_ip_service(&self) -> Arc<dyn IpService> {
        self.ip_service.clone()
    }
} 