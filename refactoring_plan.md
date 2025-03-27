# Rust DDNS Updater 重構流程表

## 第一階段：準備工作

### 1. 建立重構分支
- [ ] 創建新的 Git 分支 `refactor`
```
git checkout -b refactor
```

### 2. 增加測試覆蓋率
- [ ] 為 DDNS 服務建立單元測試
- [ ] 為 IP 檢測功能建立單元測試
- [ ] 為 API 端點建立整合測試

### 3. 分析代碼質量
- [ ] 運行 Rust 靜態分析工具 (Clippy)
```
cargo clippy
```
- [ ] 記錄當前的代碼問題清單

## 第二階段：模塊結構重構

### 1. 重構模塊架構
- [ ] 創建基礎模塊結構
```
src/
├── domain/       # 領域模型
├── application/  # 應用服務
├── infrastructure/ # 基礎設施
└── interfaces/   # 接口層
```

### 2. 遷移現有代碼
- [ ] 將 `src/config/*.rs` 重構到合適位置：
  - `ddns.rs` -> `src/domain/config/ddns.rs`
  - `settings.rs` -> `src/domain/config/settings.rs`

- [ ] 將 `src/services/*.rs` 重構到合適位置：
  - `ddns.rs` -> `src/application/ddns/service.rs`
  - `ip.rs` -> `src/infrastructure/ip/service.rs`
  - `web_ui.rs` -> `src/interfaces/web/ui.rs`
  - `ddns_config.rs` -> `src/application/config/service.rs`

- [ ] 將 `src/api/*.rs` 重構到合適位置：
  - `ip.rs` -> `src/interfaces/api/ip.rs`
  - `ddns.rs` -> `src/interfaces/api/ddns.rs`
  - `cloudflare.rs` -> `src/infrastructure/api/cloudflare.rs`
  - `ddns_config.rs` -> `src/interfaces/api/config.rs`

### 3. 更新引用關係
- [ ] 更新 `mod.rs` 文件
- [ ] 更新 `lib.rs` 導出
- [ ] 更新 `main.rs` 引用

## 第三階段：依賴注入重構

### 1. 定義接口
- [ ] 創建 HTTP 客戶端接口
```rust
// src/domain/http/client.rs
pub trait HttpClient {
    async fn get(&self, url: &str, headers: Option<HeaderMap>) -> Result<String, Error>;
    async fn put(&self, url: &str, body: &str, headers: Option<HeaderMap>) -> Result<String, Error>;
    // 其他方法...
}
```

- [ ] 創建 DNS 服務接口
```rust
// src/domain/dns/service.rs
pub trait DnsService {
    async fn update_record(&self, record: DnsRecord) -> Result<DnsUpdateResult, Error>;
    async fn get_records(&self) -> Result<Vec<DnsRecord>, Error>;
    // 其他方法...
}
```

- [ ] 創建 IP 服務接口
```rust
// src/domain/ip/service.rs
pub trait IpService {
    async fn get_ipv4(&self) -> Result<String, Error>;
    async fn get_ipv6(&self) -> Result<String, Error>;
}
```

### 2. 實現具體類
- [ ] 實現 HTTP 客戶端
```rust
// src/infrastructure/http/reqwest_client.rs
pub struct ReqwestHttpClient {
    client: reqwest::Client,
}

impl HttpClient for ReqwestHttpClient {
    // 實現方法...
}
```

- [ ] 實現 DNS 服務
```rust
// src/infrastructure/dns/cloudflare_service.rs
pub struct CloudflareDnsService {
    http_client: Box<dyn HttpClient>,
    config: CloudflareConfig,
}

impl DnsService for CloudflareDnsService {
    // 實現方法...
}
```

### 3. 重構依賴注入
- [ ] 創建應用服務工廠
```rust
// src/application/factories.rs
pub struct ServiceFactory {
    // 字段...
}

impl ServiceFactory {
    pub fn create_dns_service(&self, config: &DnsConfig) -> Box<dyn DnsService> {
        // 根據配置創建適當的服務實例
    }
    
    pub fn create_ip_service(&self) -> Box<dyn IpService> {
        // 創建 IP 服務
    }
}
```

- [ ] 更新 main.rs 中的依賴注入

## 第四階段：錯誤處理重構

### 1. 定義錯誤類型
- [ ] 創建領域錯誤
```rust
// src/domain/error.rs
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("DNS record validation failed: {0}")]
    ValidationError(String),
    // 其他領域錯誤...
}
```

- [ ] 創建基礎設施錯誤
```rust
// src/infrastructure/error.rs
#[derive(Debug, thiserror::Error)]
pub enum InfrastructureError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    // 其他基礎設施錯誤...
}
```

- [ ] 創建應用錯誤
```rust
// src/application/error.rs
#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
    // 其他應用錯誤...
}
```

### 2. 使用新的錯誤類型
- [ ] 重構 DDNS 服務錯誤處理
- [ ] 重構 API 層錯誤處理
- [ ] 更新錯誤傳播模式 (使用 `?` 代替 `unwrap` 和 `expect`)

## 第五階段：配置管理重構

### 1. 定義配置模型
- [ ] 創建強類型配置模型
```rust
// src/domain/config/models.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DdnsConfig {
    pub api_token: String,
    pub zone_id: String,
    pub record_id: String,
    pub record_name: String,
    pub update_interval: u64,
    pub ip_type: IpType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpType {
    #[serde(rename = "ipv4")]
    IPv4,
    #[serde(rename = "ipv6")]
    IPv6,
}
```

### 2. 定義配置存儲庫接口
- [ ] 創建配置存儲庫接口
```rust
// src/domain/config/repository.rs
pub trait ConfigRepository {
    fn load_configs(&self) -> Result<Vec<DdnsConfig>, ConfigError>;
    fn save_configs(&self, configs: &[DdnsConfig]) -> Result<(), ConfigError>;
}
```

### 3. 實現具體配置存儲庫
- [ ] 環境變數配置存儲庫
```rust
// src/infrastructure/config/env_repository.rs
pub struct EnvConfigRepository;

impl ConfigRepository for EnvConfigRepository {
    // 實現方法...
}
```

- [ ] JSON 文件配置存儲庫
```rust
// src/infrastructure/config/json_repository.rs
pub struct JsonConfigRepository {
    file_path: PathBuf,
}

impl ConfigRepository for JsonConfigRepository {
    // 實現方法...
}
```

## 第六階段：服務重構

### 1. 重構 DDNS 服務
- [ ] 建立 DDNS 應用服務
```rust
// src/application/ddns/service.rs
pub struct DdnsApplicationService {
    dns_service: Box<dyn DnsService>,
    ip_service: Box<dyn IpService>,
    config: DdnsConfig,
}

impl DdnsApplicationService {
    pub async fn update_record(&self) -> Result<DnsUpdateResult, ApplicationError> {
        // 實現邏輯...
    }
    
    pub async fn start_auto_update(&self) {
        // 實現邏輯...
    }
}
```

### 2. 重構 IP 服務
- [ ] 分離 IP 檢測邏輯
```rust
// src/infrastructure/ip/public_ip_service.rs
pub struct PublicIpService {
    http_client: Box<dyn HttpClient>,
}

impl IpService for PublicIpService {
    // 實現方法...
}
```

### 3. 重構 Web UI 服務
- [ ] 重新組織 Web UI 處理邏輯
```rust
// src/interfaces/web/handler.rs
pub struct WebUiHandler {
    // 依賴項...
}

impl WebUiHandler {
    // 處理方法...
}
```

## 第七階段：API 層重構

### 1. 重構 API 處理器
- [ ] 使用依賴注入重構 API 處理器
```rust
// src/interfaces/api/ip_handler.rs
pub struct IpApiHandler {
    ip_service: Box<dyn IpService>,
}

impl IpApiHandler {
    // 處理方法...
}
```

### 2. 重構路由配置
- [ ] 更新 API 路由註冊
```rust
// src/interfaces/api/router.rs
pub fn configure_routes(cfg: &mut web::ServiceConfig, service_factory: &ServiceFactory) {
    // 配置路由...
}
```

## 第八階段：狀態管理重構

### 1. 定義狀態存儲接口
- [ ] 創建狀態存儲接口
```rust
// src/domain/state/repository.rs
pub trait StateRepository {
    fn get_last_ip(&self, config_id: &str) -> Option<String>;
    fn set_last_ip(&self, config_id: &str, ip: &str);
    fn get_last_update_time(&self, config_id: &str) -> Option<DateTime<Utc>>;
    fn set_last_update_time(&self, config_id: &str, time: DateTime<Utc>);
}
```

### 2. 實現具體狀態存儲
- [ ] 內存狀態存儲
```rust
// src/infrastructure/state/memory_repository.rs
pub struct InMemoryStateRepository {
    state: Arc<RwLock<HashMap<String, StateEntry>>>,
}

impl StateRepository for InMemoryStateRepository {
    // 實現方法...
}
```

## 第九階段：日誌與監控重構

### 1. 增強日誌記錄
- [ ] 將 `log` crate 替換為 `tracing`
- [ ] 添加結構化日誌
- [ ] 增加關鍵操作的時間戳和執行時間記錄

### 2. 添加健康檢查
- [ ] 添加 `/health` 端點
```rust
// src/interfaces/api/health.rs
pub async fn health_check() -> impl Responder {
    // 實現健康檢查邏輯
}
```

## 第十階段：測試與文檔

### 1. 更新測試
- [ ] 更新單元測試以適應新的架構
- [ ] 添加集成測試

### 2. 更新文檔
- [ ] 更新代碼注釋與文檔字符串
- [ ] 更新 README.md

## 第十一階段：清理與優化

### 1. 代碼清理
- [ ] 運行 `cargo clippy` 並修復問題
- [ ] 運行 `cargo fmt` 格式化代碼

### 2. 性能優化
- [ ] 識別並優化性能熱點
- [ ] 檢查內存使用和優化

## 第十二階段：最終測試與發布

### 1. 最終測試
- [ ] 執行所有測試
- [ ] 手動功能測試

### 2. 準備發布
- [ ] 更新版本號
- [ ] 合併回主分支
```
git checkout main
git merge refactor
git tag v0.2.0
git push origin main --tags
```

## 重構時間線

| 階段 | 預估工作時間 | 優先級 |
|------|------------|-------|
| 第一階段：準備工作 | 1-2 天 | 高 |
| 第二階段：模塊結構重構 | 2-3 天 | 高 |
| 第三階段：依賴注入重構 | 2-3 天 | 高 |
| 第四階段：錯誤處理重構 | 1-2 天 | 高 |
| 第五階段：配置管理重構 | 2 天 | 中 |
| 第六階段：服務重構 | 2-3 天 | 中 |
| 第七階段：API 層重構 | 1-2 天 | 中 |
| 第八階段：狀態管理重構 | 1 天 | 低 |
| 第九階段：日誌與監控重構 | 1 天 | 低 |
| 第十階段：測試與文檔 | 2 天 | 中 |
| 第十一階段：清理與優化 | 1 天 | 低 |
| 第十二階段：最終測試與發布 | 1 天 | 高 |

**總計預估工作時間：** 17-23 工作日 