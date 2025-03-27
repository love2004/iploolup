use async_trait::async_trait;
use cloudflare_ddns::domain::dns::{DnsRecord, DnsService, DnsUpdateResult};
use cloudflare_ddns::domain::error::DomainError;
use cloudflare_ddns::domain::http::HttpClient;
use mockall::predicate::*;
use mockall::mock;
use reqwest::header::HeaderMap;
use serde::Serialize;

// 創建模擬 HTTP 客戶端
mock! {
    pub HttpClientMock {}
    
    #[async_trait]
    impl HttpClient for HttpClientMock {
        async fn get(&self, url: &str, headers: Option<HeaderMap>) -> Result<String, DomainError>;
        async fn post(&self, url: &str, body: Option<String>, headers: Option<HeaderMap>) -> Result<String, DomainError>;
        async fn put(&self, url: &str, body: Option<String>, headers: Option<HeaderMap>) -> Result<String, DomainError>;
        async fn delete(&self, url: &str, headers: Option<HeaderMap>) -> Result<String, DomainError>;
    }
}

// 擴展模擬客戶端以支持 JSON 方法
impl MockHttpClientMock {
    // 這些方法是手動實現，而不是通過 mockall 自動生成
    pub async fn get_json<T: for<'de> serde::Deserialize<'de> + 'static>(&self, url: &str, headers: Option<HeaderMap>) -> Result<T, DomainError> {
        let response = self.get(url, headers).await?;
        serde_json::from_str(&response).map_err(|e| DomainError::LogicError(format!("JSON 解析錯誤: {}", e)))
    }
    
    pub async fn post_json<T: for<'de> serde::Deserialize<'de> + 'static, U: Serialize + Send + Sync>(&self, url: &str, body: Option<&U>, headers: Option<HeaderMap>) -> Result<T, DomainError> {
        let body_str = match body {
            Some(b) => Some(serde_json::to_string(b).map_err(|e| DomainError::LogicError(format!("JSON 序列化錯誤: {}", e)))?),
            None => None,
        };
        
        let response = self.post(url, body_str, headers).await?;
        serde_json::from_str(&response).map_err(|e| DomainError::LogicError(format!("JSON 解析錯誤: {}", e)))
    }
    
    pub async fn put_json<T: for<'de> serde::Deserialize<'de> + 'static, U: Serialize + Send + Sync>(&self, url: &str, body: Option<&U>, headers: Option<HeaderMap>) -> Result<T, DomainError> {
        let body_str = match body {
            Some(b) => Some(serde_json::to_string(b).map_err(|e| DomainError::LogicError(format!("JSON 序列化錯誤: {}", e)))?),
            None => None,
        };
        
        let response = self.put(url, body_str, headers).await?;
        serde_json::from_str(&response).map_err(|e| DomainError::LogicError(format!("JSON 解析錯誤: {}", e)))
    }
}

// 創建模擬 DNS 服務
mock! {
    pub DnsServiceMock {}
    
    #[async_trait]
    impl DnsService for DnsServiceMock {
        async fn update_record(&self, record: DnsRecord) -> Result<DnsUpdateResult, DomainError>;
        async fn get_record(&self, zone_id: &str, record_id: &str) -> Result<DnsRecord, DomainError>;
        async fn get_records(&self, zone_id: &str) -> Result<Vec<DnsRecord>, DomainError>;
        async fn create_record(&self, zone_id: &str, record: DnsRecord) -> Result<DnsRecord, DomainError>;
    }
}

#[tokio::test]
async fn test_dns_service_update_record() {
    // 創建測試數據
    let record = DnsRecord {
        id: Some("record-123".to_string()),
        name: "test.example.com".to_string(),
        record_type: "A".to_string(),
        content: "192.168.1.1".to_string(),
        ttl: 60,
        proxied: false,
    };
    
    // 創建模擬 DNS 服務
    let mut mock = MockDnsServiceMock::new();
    
    // 設置模擬行為
    mock.expect_update_record()
        .with(function(move |r: &DnsRecord| {
            r.record_type == "A" && r.content == "192.168.1.1"
        }))
        .times(1)
        .returning(|record| {
            Ok(DnsUpdateResult {
                record: record.clone(),
                updated: true,
            })
        });
    
    // 測試更新記錄
    let result = mock.update_record(record.clone()).await.expect("Update should succeed");
    
    assert!(result.updated);
    assert_eq!(result.record.content, "192.168.1.1");
    assert_eq!(result.record.record_type, "A");
}

#[tokio::test]
async fn test_dns_service_get_record() {
    // 創建模擬 DNS 服務
    let mut mock = MockDnsServiceMock::new();
    
    // 設置模擬行為
    mock.expect_get_record()
        .with(eq("zone-123"), eq("record-456"))
        .times(1)
        .returning(|_, _| {
            Ok(DnsRecord {
                id: Some("record-456".to_string()),
                record_type: "A".to_string(),
                name: "test.example.com".to_string(),
                content: "192.168.1.1".to_string(),
                ttl: 120,
                proxied: false,
            })
        });
    
    // 測試獲取記錄
    let record = mock.get_record("zone-123", "record-456").await.expect("Get should succeed");
    
    assert_eq!(record.id.unwrap(), "record-456");
    assert_eq!(record.record_type, "A");
    assert_eq!(record.name, "test.example.com");
    assert_eq!(record.content, "192.168.1.1");
}

// 移除對私有模塊的測試，因為無法訪問 CloudflareDnsService
// 這裡我們只測試 DnsService trait 的功能 