use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::time::Duration;
use chrono::Utc;

use cloudflare_ddns::domain::config::{DdnsConfig, IpType};
use cloudflare_ddns::domain::dns::{DnsService, DnsRecord, DnsUpdateResult};
use cloudflare_ddns::domain::ip::IpService;
use cloudflare_ddns::domain::state::{StateRepository, StateEntry};
use cloudflare_ddns::domain::error::DomainError;
use cloudflare_ddns::application::ddns::DdnsApplicationService;

// 模擬 DNS 服務
struct MockDnsService {
    update_count: std::sync::atomic::AtomicU32,
    record_to_return: DnsRecord,
    updated_record: DnsRecord,
}

impl MockDnsService {
    fn new(record_to_return: DnsRecord, updated_record: DnsRecord) -> Self {
        Self {
            update_count: std::sync::atomic::AtomicU32::new(0),
            record_to_return,
            updated_record,
        }
    }
    
    fn get_update_count(&self) -> u32 {
        self.update_count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

#[async_trait]
impl DnsService for MockDnsService {
    async fn update_record(&self, _record: DnsRecord) -> Result<DnsUpdateResult, DomainError> {
        self.update_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        Ok(DnsUpdateResult {
            record: self.updated_record.clone(),
            updated: true,
        })
    }
    
    async fn get_record(&self, _zone_id: &str, _record_id: &str) -> Result<DnsRecord, DomainError> {
        Ok(self.record_to_return.clone())
    }
    
    async fn get_records(&self, _zone_id: &str) -> Result<Vec<DnsRecord>, DomainError> {
        Ok(vec![self.record_to_return.clone()])
    }
    
    async fn create_record(&self, _zone_id: &str, record: DnsRecord) -> Result<DnsRecord, DomainError> {
        Ok(record)
    }
}

// 模擬 IP 服務
struct MockIpService {
    ipv4: String,
    ipv6: String,
}

impl MockIpService {
    fn new(ipv4: &str, ipv6: &str) -> Self {
        Self {
            ipv4: ipv4.to_string(),
            ipv6: ipv6.to_string(),
        }
    }
}

#[async_trait]
impl IpService for MockIpService {
    async fn get_ipv4(&self) -> Result<String, DomainError> {
        Ok(self.ipv4.clone())
    }
    
    async fn get_ipv6(&self) -> Result<String, DomainError> {
        Ok(self.ipv6.clone())
    }
}

// 模擬狀態儲存庫
struct MockStateRepository {
    last_ip: std::sync::Mutex<HashMap<String, String>>,
    last_update_time: std::sync::Mutex<HashMap<String, chrono::DateTime<Utc>>>,
}

impl MockStateRepository {
    fn new() -> Self {
        Self {
            last_ip: std::sync::Mutex::new(HashMap::new()),
            last_update_time: std::sync::Mutex::new(HashMap::new()),
        }
    }
    
    fn set_last_ip(&self, config_id: &str, ip: &str) {
        let mut map = self.last_ip.lock().unwrap();
        map.insert(config_id.to_string(), ip.to_string());
    }
}

#[async_trait]
impl StateRepository for MockStateRepository {
    async fn get_last_ip(&self, config_id: &str) -> Result<Option<String>, DomainError> {
        let map = self.last_ip.lock().unwrap();
        Ok(map.get(config_id).cloned())
    }
    
    async fn set_last_ip(&self, config_id: &str, ip: &str) -> Result<(), DomainError> {
        let mut map = self.last_ip.lock().unwrap();
        map.insert(config_id.to_string(), ip.to_string());
        Ok(())
    }
    
    async fn get_last_update_time(&self, config_id: &str) -> Result<Option<chrono::DateTime<Utc>>, DomainError> {
        let map = self.last_update_time.lock().unwrap();
        Ok(map.get(config_id).cloned())
    }
    
    async fn set_last_update_time(&self, config_id: &str, time: chrono::DateTime<Utc>) -> Result<(), DomainError> {
        let mut map = self.last_update_time.lock().unwrap();
        map.insert(config_id.to_string(), time);
        Ok(())
    }
    
    async fn get_state(&self, config_id: &str) -> Result<Option<StateEntry>, DomainError> {
        let ip_map = self.last_ip.lock().unwrap();
        let time_map = self.last_update_time.lock().unwrap();
        
        let last_ip = ip_map.get(config_id).cloned();
        let last_update_time = time_map.get(config_id).cloned();
        
        if last_ip.is_none() && last_update_time.is_none() {
            return Ok(None);
        }
        
        Ok(Some(StateEntry {
            last_ip,
            last_update_time,
        }))
    }
    
    async fn set_state(&self, config_id: &str, state: StateEntry) -> Result<(), DomainError> {
        if let Some(ip) = state.last_ip {
            let mut ip_map = self.last_ip.lock().unwrap();
            ip_map.insert(config_id.to_string(), ip);
        }
        
        if let Some(time) = state.last_update_time {
            let mut time_map = self.last_update_time.lock().unwrap();
            time_map.insert(config_id.to_string(), time);
        }
        
        Ok(())
    }
}

#[tokio::test]
async fn test_ddns_service_update_record_when_ip_changed() {
    // 準備測試數據
    let config = DdnsConfig {
        api_token: "test-token".to_string(),
        zone_id: "test-zone-id".to_string(),
        record_id: "test-record-id".to_string(),
        record_name: "test.example.com".to_string(),
        update_interval: 300,
        ip_type: IpType::IPv4,
    };
    
    // 設置舊的 DNS 記錄
    let old_record = DnsRecord {
        id: Some("test-record-id".to_string()),
        name: "test.example.com".to_string(),
        content: "192.168.0.1".to_string(), // 不同的 IP
        record_type: "A".to_string(),
        ttl: 120,
        proxied: false,
    };
    
    // 設置更新後的 DNS 記錄
    let updated_record = DnsRecord {
        id: Some("test-record-id".to_string()),
        name: "test.example.com".to_string(),
        content: "192.168.1.1".to_string(), // 更新後的 IP
        record_type: "A".to_string(),
        ttl: 120,
        proxied: false,
    };
    
    // 創建模擬服務
    let dns_service = Arc::new(MockDnsService::new(old_record, updated_record));
    let ip_service = Arc::new(MockIpService::new("192.168.1.1", "2001:db8::1"));
    let state_repo = Arc::new(MockStateRepository::new());
    
    // 設置配置 ID
    let config_id = format!("{}-{}", config.zone_id, config.record_id);
    
    // 創建 DDNS 應用服務
    let ddns_service = DdnsApplicationService::new(
        dns_service.clone(),
        ip_service,
        state_repo.clone(),
        config,
    );
    
    // 執行更新
    let result = ddns_service.update_dns_record().await;
    
    // 驗證結果
    assert!(result.is_ok());
    assert_eq!(dns_service.get_update_count(), 1);
}

#[tokio::test]
async fn test_ddns_service_no_update_when_ip_same() {
    // 準備測試數據
    let config = DdnsConfig {
        api_token: "test-token".to_string(),
        zone_id: "test-zone-id".to_string(),
        record_id: "test-record-id".to_string(),
        record_name: "test.example.com".to_string(),
        update_interval: 300,
        ip_type: IpType::IPv4,
    };
    
    // 設置 DNS 記錄 (IP 與當前 IP 相同)
    let record = DnsRecord {
        id: Some("test-record-id".to_string()),
        name: "test.example.com".to_string(),
        content: "192.168.1.1".to_string(), // 與當前 IP 相同
        record_type: "A".to_string(),
        ttl: 120,
        proxied: false,
    };
    
    // 創建模擬服務
    let dns_service = Arc::new(MockDnsService::new(record.clone(), record.clone()));
    let ip_service = Arc::new(MockIpService::new("192.168.1.1", "2001:db8::1"));
    let state_repo = Arc::new(MockStateRepository::new());
    
    // 設置配置 ID
    let config_id = format!("{}-{}", config.zone_id, config.record_id);
    
    // 設置上次的 IP 地址（與當前 IP 相同）
    state_repo.set_last_ip(&config_id, &"192.168.1.1".to_string());
    
    // 創建 DDNS 應用服務
    let ddns_service = DdnsApplicationService::new(
        dns_service.clone(),
        ip_service,
        state_repo,
        config,
    );
    
    // 執行更新
    let result = ddns_service.update_dns_record().await;
    
    // 驗證結果
    assert!(result.is_ok());
    assert_eq!(dns_service.get_update_count(), 0);
    if let Ok(update_result) = result {
        assert_eq!(update_result.updated, false);
    }
}

#[tokio::test]
async fn test_ddns_service_force_update() {
    // 準備測試數據
    let config = DdnsConfig {
        api_token: "test-token".to_string(),
        zone_id: "test-zone-id".to_string(),
        record_id: "test-record-id".to_string(),
        record_name: "test.example.com".to_string(),
        update_interval: 300,
        ip_type: IpType::IPv4,
    };
    
    // 設置 DNS 記錄
    let record = DnsRecord {
        id: Some("test-record-id".to_string()),
        name: "test.example.com".to_string(),
        content: "192.168.1.1".to_string(),
        record_type: "A".to_string(),
        ttl: 120,
        proxied: false,
    };
    
    // 創建模擬服務
    let dns_service = Arc::new(MockDnsService::new(record.clone(), record.clone()));
    let ip_service = Arc::new(MockIpService::new("192.168.1.1", "2001:db8::1"));
    let state_repo = Arc::new(MockStateRepository::new());
    
    // 創建 DDNS 應用服務
    let ddns_service = DdnsApplicationService::new(
        dns_service.clone(),
        ip_service,
        state_repo,
        config,
    );
    
    // 執行強制更新
    let result = ddns_service.force_update().await;
    
    // 驗證結果
    assert!(result.is_ok());
    assert_eq!(dns_service.get_update_count(), 1);
    if let Ok((domain, ip)) = result {
        assert_eq!(domain, "test.example.com");
        assert_eq!(ip, "192.168.1.1");
    }
} 