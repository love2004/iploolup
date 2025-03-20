use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use crate::error::AppError;
use crate::services::ip;

#[derive(Serialize, Deserialize, Debug)]
pub struct DdnsConfig {
    pub api_token: String,
    pub zone_id: String,
    pub record_id: String, 
    pub record_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CloudflareResponse {
    success: bool,
    errors: Vec<String>,
    messages: Vec<String>,
    result: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdateRecord {
    #[serde(rename = "type")]
    record_type: String,
    name: String,
    content: String,
    ttl: u32,
    proxied: bool,
}

pub struct DdnsService {
    config: DdnsConfig,
    client: reqwest::Client,
}

impl DdnsService {
    pub fn new(config: DdnsConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub async fn update_record(&self) -> Result<serde_json::Value, AppError> {
        let current_ip = ip::fetch_ipv4().await?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", self.config.api_token))
            .map_err(|_| AppError::ConfigError("無效的 API token".to_string()))?);

        let update_data = UpdateRecord {
            record_type: "A".to_string(),
            name: self.config.record_name.clone(),
            content: current_ip.clone(),
            ttl: 120,
            proxied: false,
        };

        let res = self.client.put(&format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", 
            self.config.zone_id, self.config.record_id
        ))
        .headers(headers)
        .json(&update_data)
        .send()
        .await
        .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;

        let cf_response: CloudflareResponse = res.json()
            .await
            .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;

        if cf_response.success {
            Ok(serde_json::json!({
                "status": "success",
                "message": "DNS 記錄已更新",
                "data": {
                    "ip": current_ip,
                    "domain": self.config.record_name,
                    "ttl": 120,
                    "proxied": false
                }
            }))
        } else {
            Err(AppError::ExternalServiceError(format!("Cloudflare API 錯誤: {:?}", cf_response.errors)))
        }
    }
    
    pub async fn update_ipv6_record(&self) -> Result<serde_json::Value, AppError> {
        let current_ip = ip::fetch_ipv6().await?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", self.config.api_token))
            .map_err(|_| AppError::ConfigError("無效的 API token".to_string()))?);

        let update_data = UpdateRecord {
            record_type: "AAAA".to_string(),
            name: self.config.record_name.clone(),
            content: current_ip.clone(),
            ttl: 120,
            proxied: false,
        };

        let res = self.client.put(&format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", 
            self.config.zone_id, self.config.record_id
        ))
        .headers(headers)
        .json(&update_data)
        .send()
        .await
        .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;

        let cf_response: CloudflareResponse = res.json()
            .await
            .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;

        if cf_response.success {
            Ok(serde_json::json!({
                "status": "success",
                "message": "IPv6 DNS 記錄已更新",
                "data": {
                    "ip": current_ip,
                    "domain": self.config.record_name,
                    "ttl": 120,
                    "proxied": false
                }
            }))
        } else {
            Err(AppError::ExternalServiceError(format!("Cloudflare API 錯誤: {:?}", cf_response.errors)))
        }
    }
} 