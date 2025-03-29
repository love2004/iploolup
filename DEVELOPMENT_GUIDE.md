# Rust DDNS 更新服務開發指南

## 專案概述

這是一個使用 Rust 語言開發的 Cloudflare DDNS 更新服務，主要功能是自動檢測並更新 Cloudflare DNS 記錄中的 IP 地址。專案採用領域驅動設計（DDD）架構，確保代碼的可維護性和可擴展性。

## 技術棧

- **程式語言**: Rust 2021 Edition
- **Web 框架**: Actix-web 4.0
- **非同步運行時**: Tokio 1.x
- **HTTP 客戶端**: Reqwest 0.11
- **序列化/反序列化**: Serde 1.0
- **配置管理**: config 0.13
- **日誌系統**: log 0.4 + env_logger 0.10
- **錯誤處理**: thiserror 1.0
- **環境變數**: dotenv 0.15
- **測試框架**: mockall 0.11 + tokio-test 0.4

## 專案結構

```
src/
├── domain/          # 領域層：核心業務邏輯
├── application/     # 應用層：協調領域服務
├── infrastructure/  # 基礎設施層：外部服務實現
├── interfaces/      # 介面層：API 和 Web 介面
├── main.rs         # 應用程式入口
└── lib.rs          # 庫入口點
```

## 核心概念

### 領域驅動設計（DDD）架構

專案採用 DDD 架構，分為以下幾層：

1. **領域層（Domain Layer）**
   - 包含核心業務邏輯
   - 定義領域模型和服務介面
   - 不依賴外部服務

2. **應用層（Application Layer）**
   - 協調領域服務
   - 處理用例實現
   - 事務管理

3. **基礎設施層（Infrastructure Layer）**
   - 實現外部服務介面
   - 處理數據持久化
   - 提供技術實現

4. **介面層（Interface Layer）**
   - 處理 HTTP 請求
   - 提供 API 端點
   - 數據轉換和驗證

### 重要特性

1. **非同步編程**
   - 使用 `async/await` 語法
   - 基於 Tokio 運行時
   - 使用 `async-trait` 實現非同步特徵

2. **錯誤處理**
   - 使用 `thiserror` 定義自定義錯誤
   - 統一的錯誤處理機制
   - 清晰的錯誤類型層次結構

3. **配置管理**
   - 使用 `config` 庫處理配置
   - 支持多種配置源
   - 環境變數覆蓋

4. **日誌系統**
   - 結構化日誌
   - 可配置的日誌級別
   - 環境變數控制

## 開發指南

### 環境設置

1. 安裝 Rust 工具鏈：
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. 安裝依賴：
   ```bash
   cargo build
   ```

3. 配置環境變數：
   ```bash
   cp .env.example .env
   # 編輯 .env 文件設置必要的配置
   ```

### 開發流程

1. **添加新功能**
   - 在領域層定義介面
   - 在應用層實現用例
   - 在基礎設施層提供實現
   - 在介面層暴露 API

2. **測試**
   - 使用 `mockall` 進行模擬
   - 使用 `tokio-test` 測試非同步代碼
   - 遵循單元測試和整合測試最佳實踐

3. **錯誤處理**
   - 在領域層定義錯誤類型
   - 使用 `thiserror` 派生錯誤實現
   - 提供清晰的錯誤信息

### 代碼風格

1. **命名規範**
   - 使用有意義的名稱
   - 遵循 Rust 命名慣例
   - 使用清晰的模組結構

2. **文檔**
   - 為公共 API 提供文檔註釋
   - 使用示例說明複雜邏輯
   - 保持 README 更新

3. **錯誤處理**
   - 使用 Result 類型處理錯誤
   - 提供有幫助的錯誤信息
   - 適當使用錯誤轉換

## 部署

### Docker 部署

1. 構建映像：
   ```bash
   docker-compose build
   ```

2. 運行服務：
   ```bash
   docker-compose up -d
   ```

### 環境變數

主要環境變數：
- `RUST_LOG`: 日誌級別
- `CONFIG_FILE`: 配置文件路徑
- `PORT`: 服務端口
- `CLOUDFLARE_API_TOKEN`: Cloudflare API 令牌

## 常見問題

1. **非同步編程注意事項**
   - 注意 `.await` 的使用
   - 避免阻塞操作
   - 正確處理生命週期

2. **錯誤處理最佳實踐**
   - 使用 `?` 運算符
   - 適當使用 `map_err`
   - 保持錯誤類型一致

3. **配置管理**
   - 優先使用環境變數
   - 提供合理的默認值
   - 驗證配置有效性

## 維護指南

1. **日誌管理**
   - 定期檢查日誌
   - 適當設置日誌級別
   - 監控錯誤模式

2. **性能優化**
   - 監控資源使用
   - 優化數據庫查詢
   - 適當使用緩存

3. **安全考慮**
   - 定期更新依賴
   - 檢查安全漏洞
   - 保護敏感信息

## 貢獻指南

1. Fork 專案
2. 創建特性分支
3. 提交更改
4. 發起 Pull Request

## 聯繫方式

如有問題，請聯繫專案維護者。

## 主程序實現細節

### 程序架構

主程序（`main.rs`）實現了以下核心功能：

1. **多模式運行**
   - DDNS 服務模式
   - Web 服務器模式
   - 混合模式（同時運行兩種服務）

2. **服務控制**
   - 使用 `once_cell` 實現全局服務控制
   - 支持動態重啟 DDNS 服務
   - 進程間通信使用 `mpsc` 通道

3. **配置管理**
   - 從環境變數加載配置
   - 支持多個 DDNS 配置
   - 動態更新配置

### 關鍵實現

1. **全局服務控制**
   ```rust
   static DDNS_CONTROL: once_cell::sync::Lazy<Arc<Mutex<Option<mpsc::Sender<()>>>>> = 
       once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));
   ```

2. **服務重啟機制**
   ```rust
   pub fn restart_ddns_service() {
       if let Ok(mut ctrl) = DDNS_CONTROL.lock() {
           if let Some(sender) = ctrl.take() {
               let _ = sender.send(());
           }
           start_ddns_service();
       }
   }
   ```

3. **進程管理**
   - 使用 `process::Command` 管理子進程
   - 環境變數傳遞
   - 進程狀態監控

### 運行模式

1. **DDNS 模式**
   - 專注於 IP 地址更新
   - 定期檢查 IP 變化
   - 自動更新 DNS 記錄

2. **Web 模式**
   - 提供 HTTP API
   - 狀態監控
   - 配置管理介面

3. **混合模式**
   - 同時運行兩種服務
   - 資源共享
   - 協調管理

### 錯誤處理

1. **進程錯誤**
   - 子進程啟動失敗處理
   - 進程通信錯誤處理
   - 資源清理

2. **配置錯誤**
   - 環境變數缺失處理
   - 配置格式驗證
   - 動態更新錯誤處理

3. **服務錯誤**
   - 服務啟動失敗處理
   - 運行時錯誤恢復
   - 日誌記錄

## 領域層實現細節

### 領域模型

領域層（`src/domain/`）包含以下核心模組：

1. **IP 服務（`ip/`）**
   - 定義 IP 地址獲取介面
   - 支持 IPv4 和 IPv6
   - 使用非同步特徵

2. **DNS 服務（`dns/`）**
   - DNS 記錄管理
   - Cloudflare API 整合
   - 記錄更新邏輯

3. **HTTP 服務（`http/`）**
   - HTTP 客戶端介面
   - 請求處理
   - 響應解析

4. **配置管理（`config/`）**
   - 配置模型
   - 驗證邏輯
   - 默認值處理

5. **狀態管理（`state/`）**
   - 應用狀態
   - 狀態轉換
   - 持久化

### 錯誤處理

使用 `thiserror` 實現統一的錯誤處理：

```rust
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Entity not found: {0}")]
    NotFoundError(String),
    
    #[error("Unauthorized action: {0}")]
    UnauthorizedError(String),
    
    #[error("Resource conflict: {0}")]
    ConflictError(String),
    
    #[error("Domain logic error: {0}")]
    LogicError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}
```

### 服務介面

1. **IP 服務介面**
   ```rust
   #[async_trait]
   pub trait IpService: Send + Sync {
       async fn get_ipv4(&self) -> Result<String, DomainError>;
       async fn get_ipv6(&self) -> Result<String, DomainError>;
   }
   ```

2. **特徵實現**
   - 使用 `async-trait` 實現非同步特徵
   - 確保線程安全（`Send + Sync`）
   - 統一的錯誤處理

### 領域規則

1. **IP 地址管理**
   - 定期檢查 IP 變化
   - 支持多種 IP 獲取方式
   - 緩存機制

2. **DNS 更新規則**
   - 只在 IP 變化時更新
   - 批量更新支持
   - 失敗重試機制

3. **配置驗證**
   - 必填字段檢查
   - 格式驗證
   - 默認值處理

### 最佳實踐

1. **介面設計**
   - 最小化介面
   - 清晰的文檔
   - 合理的錯誤處理

2. **實現原則**
   - 依賴注入
   - 單一職責
   - 開閉原則

3. **測試策略**
   - 單元測試
   - 模擬測試
   - 整合測試

## 應用層實現細節

### 事件系統

應用層實現了一個完整的事件驅動系統：

1. **事件類型**
   ```rust
   #[derive(Debug, Clone, PartialEq, Eq, Hash)]
   pub enum EventType {
       RestartDdnsService,
       ForceUpdateDns,
       ConfigChanged,
   }
   ```

2. **事件數據結構**
   ```rust
   pub struct EventData {
       pub event_type: EventType,
       pub data: Option<String>,
   }
   ```

3. **事件監聽器介面**
   ```rust
   #[async_trait]
   pub trait EventListener: Send + Sync {
       async fn handle_event(&self, event: EventData);
   }
   ```

### 服務工廠

`ServiceFactory` 負責創建和管理所有服務實例：

1. **核心組件**
   ```rust
   pub struct ServiceFactory {
       http_client: Arc<ReqwestHttpClient>,
       ip_service: Arc<dyn IpService>,
       state_repository: Arc<dyn StateRepository>,
       ddns_services: Arc<RwLock<HashMap<u64, Arc<Mutex<DdnsApplicationService>>>>>,
       event_manager: Arc<EventManager>,
       config_service: Arc<ConfigService>,
   }
   ```

2. **服務管理**
   - 動態創建 DDNS 服務
   - 配置加載和重載
   - 服務生命週期管理

3. **事件處理**
   - 服務重啟
   - DNS 記錄更新
   - 配置變更

### 應用服務

1. **DDNS 應用服務**
   - 協調 IP 和 DNS 服務
   - 處理更新邏輯
   - 狀態管理

2. **配置服務**
   - 配置驗證
   - 配置持久化
   - 動態更新

### 依賴注入

1. **服務創建**
   ```rust
   pub fn create_dns_service(&self, config: &DdnsConfig) -> Arc<dyn DnsService> {
       Arc::new(CloudflareDnsService::new(
           self.http_client.clone(),
           config.clone(),
       ))
   }
   ```

2. **服務查找**
   ```rust
   pub async fn find_ddns_service(&self, record_name: &str) -> Option<Arc<Mutex<DdnsApplicationService>>> {
       let services = self.ddns_services.read().await;
       services.values().find(|service| {
           // 服務查找邏輯
       }).cloned()
   }
   ```

### 錯誤處理

1. **應用錯誤**
   - 服務創建錯誤
   - 配置錯誤
   - 運行時錯誤

2. **錯誤恢復**
   - 服務重試
   - 配置回滾
   - 狀態恢復

### 最佳實踐

1. **服務管理**
   - 使用 Arc 共享所有權
   - 使用 RwLock 處理並發
   - 適當的錯誤處理

2. **事件處理**
   - 異步事件處理
   - 事件過濾
   - 錯誤恢復

3. **配置管理**
   - 配置驗證
   - 動態更新
   - 持久化

## 基礎設施層實現細節

### HTTP 客戶端

使用 Reqwest 實現 HTTP 客戶端：

1. **基本實現**
   ```rust
   pub struct ReqwestHttpClient {
       client: reqwest::Client,
   }
   ```

2. **HTTP 方法**
   ```rust
   #[async_trait]
   impl HttpClient for ReqwestHttpClient {
       async fn get(&self, url: &str, headers: Option<HeaderMap>) -> Result<String, DomainError>
       async fn post(&self, url: &str, body: Option<String>, headers: Option<HeaderMap>) -> Result<String, DomainError>
       async fn put(&self, url: &str, body: Option<String>, headers: Option<HeaderMap>) -> Result<String, DomainError>
       async fn delete(&self, url: &str, headers: Option<HeaderMap>) -> Result<String, DomainError>
   }
   ```

3. **JSON 支持**
   ```rust
   impl HttpClientExt for ReqwestHttpClient {
       async fn get_json<T: DeserializeOwned + Send + 'static>(&self, url: &str, headers: Option<HeaderMap>) -> Result<T, DomainError>
       async fn post_json<T: DeserializeOwned + Send + 'static, U: Serialize + Send + Sync>(&self, url: &str, body: Option<&U>, headers: Option<HeaderMap>) -> Result<T, DomainError>
   }
   ```

### IP 服務實現

公共 IP 查詢服務：

1. **服務結構**
   ```rust
   pub struct PublicIpService {
       http_client: Arc<ReqwestHttpClient>,
       ipv4_url: String,
       ipv6_url: String,
   }
   ```

2. **配置選項**
   - 可配置的 IPv4 查詢 URL
   - 可配置的 IPv6 查詢 URL
   - 默認使用 ipify.org 服務

3. **實現細節**
   ```rust
   #[async_trait]
   impl IpService for PublicIpService {
       async fn get_ipv4(&self) -> Result<String, DomainError> {
           self.http_client.get(&self.ipv4_url, None).await
       }
       
       async fn get_ipv6(&self) -> Result<String, DomainError> {
           self.http_client.get(&self.ipv6_url, None).await
       }
   }
   ```

### DNS 服務實現

Cloudflare DNS 服務：

1. **API 整合**
   - Cloudflare API 認證
   - DNS 記錄管理
   - 批量操作支持

2. **記錄管理**
   - 創建記錄
   - 更新記錄
   - 刪除記錄
   - 查詢記錄

3. **錯誤處理**
   - API 錯誤處理
   - 重試機制
   - 錯誤轉換

### 狀態管理

1. **內存狀態**
   - 使用 HashMap 存儲
   - 線程安全訪問
   - 狀態持久化

2. **配置管理**
   - 環境變數加載
   - 配置文件解析
   - 動態更新

### 最佳實踐

1. **HTTP 客戶端**
   - 連接池管理
   - 超時處理
   - 重試策略

2. **服務實現**
   - 依賴注入
   - 錯誤處理
   - 資源管理

3. **性能優化**
   - 連接復用
   - 緩存機制
   - 並發處理

## 介面層實現細節

### API 路由

使用 Actix-web 框架實現 RESTful API：

1. **路由配置**
   ```rust
   pub fn configure_routes(cfg: &mut web::ServiceConfig) {
       cfg.service(
           web::scope("/api")
               .service(
                   web::scope("/ip")
                       .service(get_ipv4)
                       .service(get_ipv6)
               )
               .service(health_check)
               .service(get_status)
               .service(force_update)
               .service(restart_service)
               .service(get_configs)
               .service(save_configs)
               .service(validate_config)
       );
   }
   ```

2. **API 端點**
   - `/api/ip`: IP 地址查詢
   - `/api/status`: 服務狀態
   - `/api/update`: 更新操作
   - `/api/config`: 配置管理

### 狀態 API

服務狀態查詢實現：

1. **響應結構**
   ```rust
   #[derive(Serialize)]
   pub struct StatusResponse {
       status: String,
       version: String,
       last_update: Option<String>,
       ip_address: Option<String>,
       domain: Option<String>,
   }
   ```

2. **處理邏輯**
   ```rust
   #[get("/status")]
   pub async fn get_status(service_factory: web::Data<Arc<ServiceFactory>>) -> impl Responder {
       // 獲取服務狀態
       // 返回狀態信息
   }
   ```

### 配置 API

配置管理實現：

1. **請求/響應結構**
   ```rust
   #[derive(Serialize)]
   struct ConfigResponse {
       success: bool,
       message: String,
       configs: Option<Vec<DdnsConfig>>,
   }

   #[derive(Deserialize)]
   pub struct SaveConfigRequest {
       configs: Vec<DdnsConfig>,
   }
   ```

2. **配置操作**
   - 獲取配置
   - 保存配置
   - 驗證配置

### 錯誤處理

1. **HTTP 錯誤**
   - 400 Bad Request
   - 401 Unauthorized
   - 404 Not Found
   - 500 Internal Server Error

2. **錯誤響應**
   ```rust
   HttpResponse::BadRequest().json(json!({
       "success": false,
       "message": "錯誤信息"
   }))
   ```

### 安全性

1. **認證**
   - API 令牌驗證
   - 請求限流
   - CORS 配置

2. **數據驗證**
   - 輸入驗證
   - 配置驗證
   - 錯誤處理

### 最佳實踐

1. **API 設計**
   - RESTful 原則
   - 版本控制
   - 文檔化

2. **性能優化**
   - 響應緩存
   - 異步處理
   - 資源管理

3. **監控**
   - 請求日誌
   - 錯誤追蹤
   - 性能指標 