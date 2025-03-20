use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use crate::error::AppError;
use crate::services::ip;
use tokio::time::{sleep, Duration};
use log::{info, error, debug};

/// DDNS 配置結構
/// 
/// # 欄位
/// 
/// - `api_token`: Cloudflare API 令牌
/// - `zone_id`: Cloudflare 區域 ID
/// - `record_id`: DNS 記錄 ID
/// - `record_name`: DNS 記錄名稱
/// - `update_interval`: 更新間隔（秒）
/// - `ip_type`: IP 類型（ipv4 或 ipv6）
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DdnsConfig {
    pub api_token: String,
    pub zone_id: String,
    pub record_id: String, 
    pub record_name: String,
    pub update_interval: u64,  // 更新間隔（秒）
    pub ip_type: String,
}

/// Cloudflare API 響應結構
/// 
/// # 欄位
/// 
/// - `success`: 請求是否成功
/// - `errors`: 錯誤訊息列表
/// - `messages`: 提示訊息列表
/// - `result`: API 響應結果
#[derive(Serialize, Deserialize, Debug)]
struct CloudflareResponse {
    success: bool,
    errors: Vec<String>,
    messages: Vec<String>,
    result: Option<serde_json::Value>,
}

/// DNS 記錄更新結構
/// 
/// # 欄位
/// 
/// - `record_type`: 記錄類型（A 或 AAAA）
/// - `name`: 記錄名稱
/// - `content`: 記錄內容（IP 地址）
/// - `ttl`: 記錄 TTL（秒）
/// - `proxied`: 是否啟用 Cloudflare 代理
#[derive(Serialize, Deserialize, Debug)]
struct UpdateRecord {
    #[serde(rename = "type")]
    record_type: String,
    name: String,
    content: String,
    ttl: u32,
    proxied: bool,
}

/// DDNS 服務結構
/// 
/// # 欄位
/// 
/// - `config`: DDNS 配置
/// - `client`: HTTP 客戶端
pub struct DdnsService {
    config: DdnsConfig,
    client: reqwest::Client,
}

impl DdnsService {
    /// 創建新的 DDNS 服務實例
    /// 
    /// # 參數
    /// 
    /// - `config`: DDNS 配置
    /// 
    /// # 返回
    /// 
    /// 新的 DDNS 服務實例
    pub fn new(config: DdnsConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 更新 DNS 記錄
    /// 
    /// # 返回
    /// 
    /// - `Result<serde_json::Value, AppError>`: 成功時返回更新結果，失敗時返回錯誤
    /// 
    /// # 錯誤
    /// 
    /// 當以下情況發生時返回錯誤：
    /// - 獲取當前 IP 失敗
    /// - API 請求失敗
    /// - 響應解析失敗
    pub async fn update_record(&self) -> Result<serde_json::Value, AppError> {
        // 根據 IP 類型獲取當前 IP
        let (current_ip, record_type) = match self.config.ip_type.as_str() {
            "ipv4" => {
                let ip = ip::fetch_ipv4().await?;
                debug!("Current IPv4 address: {}", ip);
                (ip, "A")
            },
            "ipv6" => {
                let ip = ip::fetch_ipv6().await?;
                debug!("Current IPv6 address: {}", ip);
                (ip, "AAAA")
            },
            _ => return Err(AppError::ConfigError(format!("Invalid IP type: {}", self.config.ip_type)))
        };

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", self.config.api_token))
            .map_err(|_| AppError::ConfigError("Invalid API token".to_string()))?);

        let update_data = UpdateRecord {
            record_type: record_type.to_string(),
            name: self.config.record_name.clone(),
            content: current_ip.clone(),
            ttl: 120,
            proxied: false,
        };
        info!("Preparing to update {} DNS record: {:?}", self.config.ip_type, update_data);

        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", 
            self.config.zone_id, self.config.record_id
        );
        debug!("Sending request to: {}", url);

        let res = self.client.put(&url)
            .headers(headers)
            .json(&update_data)
            .send()
            .await
            .map_err(|e| {
                error!("API request failed: {}", e);
                AppError::ExternalServiceError(e.to_string())
            })?;

        let cf_response: CloudflareResponse = res.json()
            .await
            .map_err(|e| {
                error!("Failed to parse API response: {}", e);
                AppError::ExternalServiceError(e.to_string())
            })?;

        if cf_response.success {
            let result = serde_json::json!({
                "status": "success",
                "message": format!("{} DNS record updated", self.config.ip_type),
                "data": {
                    "ip": current_ip,
                    "domain": self.config.record_name,
                    "ttl": 120,
                    "proxied": false
                }
            });
            info!("Cloudflare API returned successful response");
            Ok(result)
        } else {
            let error_msg = format!("Cloudflare API error: {:?}", cf_response.errors);
            error!("Failed to update {} DNS record: {}", self.config.ip_type, error_msg);
            Err(AppError::ExternalServiceError(error_msg))
        }
    }

    /// 開始自動更新 DNS 記錄
    /// 
    /// # 功能
    /// 
    /// 定期檢查 IP 是否變更，並在變更時更新 DNS 記錄
    /// 
    /// # 行為
    /// 
    /// - 根據配置的間隔定期檢查 IP
    /// - 只在 IP 變更時更新 DNS 記錄
    /// - 錯誤時會等待後重試
    pub async fn start_auto_update(&self) {
        let interval = Duration::from_secs(self.config.update_interval);
        let mut last_ip = String::new();
        
        info!("Starting {} DDNS auto-update service, update interval: {} seconds", self.config.ip_type, self.config.update_interval);
        
        loop {
            // 檢查 IP 是否變更
            let current_ip = match self.config.ip_type.as_str() {
                "ipv4" => match ip::fetch_ipv4().await {
                    Ok(ip) => {
                        debug!("Successfully obtained IPv4 address: {}", ip);
                        ip
                    }
                    Err(e) => {
                        error!("Failed to get IPv4: {}, retrying in 60 seconds", e);
                        sleep(Duration::from_secs(60)).await;
                        continue;
                    }
                },
                "ipv6" => match ip::fetch_ipv6().await {
                    Ok(ip) => {
                        info!("Successfully obtained IPv6 address: {}", ip);
                        ip
                    }
                    Err(e) => {
                        error!("Failed to get IPv6: {}, retrying in 60 seconds", e);
                        sleep(Duration::from_secs(60)).await;
                        continue;
                    }
                },
                _ => {
                    error!("Invalid IP type: {}, will retry in {} seconds", self.config.ip_type, self.config.update_interval);
                    sleep(interval).await;
                    continue;
                }
            };
            
            // 如果 IP 有變更，或者這是第一次檢查，更新 DNS 記錄
            if last_ip.is_empty() {
                info!("{} Initial check, current IP: {}", self.config.ip_type, current_ip);
            } else if last_ip != current_ip {
                info!("{} IP has changed from {} to {}", self.config.ip_type, last_ip, current_ip);
            } else {
                info!("{} unchanged ({}), skipping update, will check again in {} seconds", self.config.ip_type, current_ip, self.config.update_interval);
                sleep(interval).await;
                continue;
            }
            
            let update_result = self.update_record().await;
            
            match update_result {
                Ok(result) => {
                    info!("Successfully updated {} DDNS: {}, will check again in {} seconds", 
                        self.config.ip_type,
                        serde_json::to_string(&result).unwrap_or_else(|_| format!("{:?}", result)),
                        self.config.update_interval
                    );
                    last_ip = current_ip;
                }
                Err(e) => {
                    error!("Failed to update {} DDNS: {}, retrying in 60 seconds", self.config.ip_type, e);
                    sleep(Duration::from_secs(60)).await;
                    continue;
                }
            }
            
            info!("{} DDNS update completed, entering sleep mode, will check again in {} seconds", self.config.ip_type, self.config.update_interval);
            sleep(interval).await;
        }
    }
} 