use crate::domain::error::DomainError;
use crate::domain::http::HttpClient;
use crate::domain::ip::IpService;
use crate::infrastructure::http::ReqwestHttpClient;
use async_trait::async_trait;
use std::sync::Arc;

/// 公共 IP 查詢服務實現
pub struct PublicIpService {
    http_client: Arc<ReqwestHttpClient>,
    ipv4_url: String,
    ipv6_url: String,
}

impl PublicIpService {
    /// 創建新的公共 IP 查詢服務
    ///
    /// # 參數
    ///
    /// - `http_client`: HTTP 客戶端
    /// - `ipv4_url`: IPv4 查詢服務的 URL（可選，默認為 https://api4.ipify.org）
    /// - `ipv6_url`: IPv6 查詢服務的 URL（可選，默認為 https://api6.ipify.org）
    pub fn new(http_client: Arc<ReqwestHttpClient>, ipv4_url: Option<String>, ipv6_url: Option<String>) -> Self {
        Self {
            http_client,
            ipv4_url: ipv4_url.unwrap_or_else(|| "https://api4.ipify.org".to_string()),
            ipv6_url: ipv6_url.unwrap_or_else(|| "https://api6.ipify.org".to_string()),
        }
    }
}

#[async_trait]
impl IpService for PublicIpService {
    async fn get_ipv4(&self) -> Result<String, DomainError> {
        self.http_client.get(&self.ipv4_url, None).await
    }
    
    async fn get_ipv6(&self) -> Result<String, DomainError> {
        self.http_client.get(&self.ipv6_url, None).await
    }
} 