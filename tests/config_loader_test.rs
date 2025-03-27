use cloudflare_ddns::domain::config::{DdnsConfig, IpType};
use cloudflare_ddns::domain::error::DomainError;

/// 測試 DdnsConfig 結構
#[test]
fn test_ddns_config_creation() {
    // 創建配置
    let config = DdnsConfig {
        api_token: "test_token".to_string(),
        zone_id: "test_zone_id".to_string(),
        record_id: "test_record_id".to_string(),
        record_name: "test.domain.com".to_string(),
        update_interval: 300,
        ip_type: IpType::IPv4,
    };
    
    // 驗證配置欄位
    assert_eq!(config.api_token, "test_token");
    assert_eq!(config.zone_id, "test_zone_id");
    assert_eq!(config.record_id, "test_record_id");
    assert_eq!(config.record_name, "test.domain.com");
    assert_eq!(config.update_interval, 300);
    assert_eq!(config.ip_type, IpType::IPv4);
}

/// 測試配置驗證
#[test]
fn test_ddns_config_validation() {
    // 有效配置
    let valid_config = DdnsConfig {
        api_token: "test_token".to_string(),
        zone_id: "test_zone_id".to_string(),
        record_id: "test_record_id".to_string(),
        record_name: "test.domain.com".to_string(),
        update_interval: 300,
        ip_type: IpType::IPv4,
    };
    
    assert!(valid_config.validate().is_ok(), "Valid config should not have validation errors");
    
    // 無效配置 - 空 API 令牌
    let invalid_config = DdnsConfig {
        api_token: "".to_string(),
        zone_id: "test_zone_id".to_string(),
        record_id: "test_record_id".to_string(),
        record_name: "test.domain.com".to_string(),
        update_interval: 300,
        ip_type: IpType::IPv4,
    };
    
    let result = invalid_config.validate();
    assert!(result.is_err(), "Invalid config should have validation errors");
    if let Err(DomainError::ValidationError(message)) = result {
        assert!(message.contains("token"), "Error message should mention token");
    } else {
        panic!("Expected ValidationError");
    }
    
    // 無效配置 - 更新間隔太短
    let invalid_config = DdnsConfig {
        api_token: "test_token".to_string(),
        zone_id: "test_zone_id".to_string(),
        record_id: "test_record_id".to_string(),
        record_name: "test.domain.com".to_string(),
        update_interval: 3,
        ip_type: IpType::IPv4,
    };
    
    let result = invalid_config.validate();
    assert!(result.is_err(), "Invalid config should have validation errors");
    if let Err(DomainError::ValidationError(message)) = result {
        assert!(message.contains("interval"), "Error message should mention interval");
    } else {
        panic!("Expected ValidationError");
    }
}

/// 測試 IpType 枚舉
#[test]
fn test_iptype_display() {
    assert_eq!(IpType::IPv4.to_string(), "ipv4");
    assert_eq!(IpType::IPv6.to_string(), "ipv6");
}

/// 測試 IpType 轉換
#[test]
fn test_iptype_try_from() {
    assert_eq!(IpType::try_from("ipv4").unwrap(), IpType::IPv4);
    assert_eq!(IpType::try_from("ipv6").unwrap(), IpType::IPv6);
    
    let result = IpType::try_from("invalid");
    assert!(result.is_err());
    
    if let Err(DomainError::ValidationError(message)) = result {
        assert!(message.contains("Invalid IP type"), "Error message should mention invalid type");
    } else {
        panic!("Expected ValidationError");
    }
} 