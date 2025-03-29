# Rust DDNS 代碼實現指南

## Rust 特殊語法與特性

### 1. 非同步編程

#### async-trait 使用
```rust
#[async_trait]
pub trait IpService: Send + Sync {
    async fn get_ipv4(&self) -> Result<String, DomainError>;
}
```
- 使用 `async-trait` 宏實現非同步特徵
- 需要實現 `Send + Sync` 確保線程安全
- 返回 `Result` 類型處理錯誤

#### 生命週期管理
```rust
pub async fn get_json<T: DeserializeOwned + Send + 'static>(
    &self, 
    url: &str, 
    headers: Option<HeaderMap>
) -> Result<T, DomainError>
```
- 使用 `'static` 約束確保泛型類型具有靜態生命週期
- 使用 `Send` 約束確保可以在線程間移動

### 2. 錯誤處理

#### thiserror 使用
```rust
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Entity not found: {0}")]
    NotFoundError(String),
}
```
- 使用 `thiserror` 派生錯誤類型
- 自動實現 `Display` 和 `Error` 特徵
- 支持格式化錯誤信息

#### 錯誤轉換
```rust
.map_err(|e| DomainError::LogicError(format!("HTTP request failed: {}", e)))
```
- 使用 `map_err` 轉換錯誤類型
- 保持錯誤信息上下文

### 3. 並發控制

#### Arc 和 Mutex 使用
```rust
static DDNS_CONTROL: once_cell::sync::Lazy<Arc<Mutex<Option<mpsc::Sender<()>>>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));
```
- `Arc` 用於共享所有權
- `Mutex` 用於互斥訪問
- `once_cell` 用於延遲初始化

#### RwLock 使用
```rust
ddns_services: Arc<RwLock<HashMap<u64, Arc<Mutex<DdnsApplicationService>>>>>
```
- 允許多個讀者同時訪問
- 寫者獨占訪問
- 避免死鎖

### 4. 序列化/反序列化

#### Serde 使用
```rust
#[derive(Serialize, Deserialize)]
pub struct ConfigResponse {
    success: bool,
    message: String,
    configs: Option<Vec<DdnsConfig>>,
}
```
- 使用 `Serialize` 和 `Deserialize` 派生宏
- 支持可選字段
- 自動處理類型轉換

### 5. 配置管理

#### 環境變數處理
```rust
use dotenv::dotenv;
use std::env;

dotenv().ok();
let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
```
- 使用 `dotenv` 加載環境變數
- 提供默認值
- 錯誤處理

### 6. HTTP 客戶端

#### Reqwest 使用
```rust
pub struct ReqwestHttpClient {
    client: reqwest::Client,
}

impl ReqwestHttpClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}
```
- 使用連接池
- 支持超時設置
- 自動重試

### 7. 日誌系統

#### 日誌記錄
```rust
use log::{info, error, warn};

info!("服務啟動成功");
error!("更新失敗: {}", error);
warn!("配置未找到，使用默認值");
```
- 不同級別的日誌
- 格式化輸出
- 環境變數控制

### 8. 測試

#### 模擬測試
```rust
use mockall::predicate::*;

#[mockall::automock]
pub trait IpService: Send + Sync {
    async fn get_ipv4(&self) -> Result<String, DomainError>;
}

#[tokio::test]
async fn test_ip_service() {
    let mut mock = MockIpService::new();
    mock.expect_get_ipv4()
        .returning(|| Ok("1.2.3.4".to_string()));
}
```
- 使用 `mockall` 創建模擬
- 設置期望行為
- 驗證調用

### 9. 進程管理

#### 子進程控制
```rust
use std::process::Command;

let child = Command::new(env::current_exe().unwrap())
    .env("RUST_LOG", env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
    .env("RUN_MODE", "ddns")
    .spawn()?;
```
- 環境變數傳遞
- 進程狀態監控
- 錯誤處理

### 10. 事件系統

#### 事件處理
```rust
#[async_trait]
impl EventListener for DdnsServiceEventListener {
    async fn handle_event(&self, event: EventData) {
        match event.event_type {
            EventType::RestartDdnsService => {
                info!("處理重啟 DDNS 服務事件");
                self.service_factory.restart_all_ddns_services().await;
            },
            // ...
        }
    }
}
```
- 模式匹配
- 異步處理
- 錯誤恢復

## 代碼組織最佳實踐

### 1. 模組結構
```rust
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;
```
- 清晰的模組層次
- 公共接口暴露
- 內部實現隱藏

### 2. 特徵設計
```rust
pub trait HttpClient: Send + Sync {
    async fn get(&self, url: &str, headers: Option<HeaderMap>) -> Result<String, DomainError>;
}
```
- 最小化接口
- 明確的約束
- 合理的抽象

### 3. 錯誤處理鏈
```rust
async fn update_dns_record(&self) -> Result<(), DomainError> {
    self.validate_config()
        .await?
        .check_ip_change()
        .await?
        .update_record()
        .await
}
```
- 使用 `?` 運算符
- 清晰的錯誤傳播
- 適當的錯誤轉換

### 4. 資源管理
```rust
impl Drop for DdnsService {
    fn drop(&mut self) {
        // 清理資源
        self.stop_service();
    }
}
```
- 實現 `Drop` 特徵
- 自動資源清理
- 防止資源洩漏

## 性能優化技巧

### 1. 連接池
```rust
pub struct ReqwestHttpClient {
    client: reqwest::Client,
}
```
- 重用連接
- 減少開銷
- 提高性能

### 2. 緩存機制
```rust
pub struct IpCache {
    ip: Option<String>,
    last_update: DateTime<Utc>,
}
```
- 減少請求
- 提高響應速度
- 控制更新頻率

### 3. 並發處理
```rust
pub async fn update_all_records(&self) -> Result<(), DomainError> {
    let futures: Vec<_> = self.configs.iter()
        .map(|config| self.update_record(config))
        .collect();
    futures::future::join_all(futures).await;
}
```
- 並行處理
- 提高效率
- 資源控制

## 調試技巧

### 1. 日誌級別
```rust
RUST_LOG=debug cargo run
```
- 環境變數控制
- 不同級別日誌
- 問題診斷

### 2. 斷點調試
```rust
#[cfg(debug_assertions)]
{
    println!("調試信息: {:?}", value);
}
```
- 條件編譯
- 調試信息
- 性能影響

### 3. 錯誤追蹤
```rust
error!("操作失敗: {:?}", error);
```
- 詳細錯誤信息
- 上下文記錄
- 問題定位 