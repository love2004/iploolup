# Rust DDNS 更新器

這是一個用 Rust 實現的 DDNS (動態 DNS) 更新工具，目前主要支援 Cloudflare DNS 服務。該應用程式可以自動檢測您的公共 IP 地址並更新對應的 DNS 記錄。

## 功能特點

- 支援 IPv4 和 IPv6 地址更新
- 支援 Cloudflare DNS API
- 可配置的更新間隔
- 內建 WebUI 用於監控和管理
- HTTP 請求自動重試機制
- 結構化錯誤處理系統
- 支援多個 DNS 記錄同時更新
- 豐富的日誌記錄

## 系統要求

- Rust 1.67.0 或更高版本

## 安裝

從源碼編譯安裝：

```bash
git clone https://github.com/yourusername/cloudflare-ddns.git
cd cloudflare-ddns
cargo build --release
```

## 配置

應用程式支援以下配置方式：

1. 通過環境變數
2. 通過配置文件（TOML, JSON, YAML）
3. 通過命令行參數

### 環境變數配置

```bash
# 必需的配置項
export CLOUDFLARE_API_TOKEN=your_api_token
export CLOUDFLARE_ZONE_ID=your_zone_id
export CLOUDFLARE_RECORD_ID=your_record_id
export CLOUDFLARE_RECORD_NAME=your.domain.com

# 可選配置項
export UPDATE_INTERVAL=300  # 更新間隔（秒）
export IP_TYPE=IPv4         # 可選: IPv4, IPv6, Both
```

### 配置文件（config.toml）

```toml
[cloudflare]
api_token = "your_api_token"
zone_id = "your_zone_id"
record_id = "your_record_id"
record_name = "your.domain.com"
update_interval = 300
ip_type = "IPv4"  # 可選: IPv4, IPv6, Both
```

## 使用方法

直接運行可執行文件：

```bash
./target/release/cloudflare-ddns
```

使用配置文件：

```bash
./target/release/cloudflare-ddns --config config.toml
```

## 開發與測試

運行測試：

```bash
cargo test
```

運行特定測試：

```bash
cargo test test_http_client_retry_mechanism
```

執行集成測試：

```bash
cargo test --test integration_tests
```

## 架構設計

該項目採用領域驅動設計（DDD）的架構：

- **領域層 (Domain)**: 包含核心業務邏輯和實體
- **應用層 (Application)**: 協調領域物件完成用戶任務
- **基礎設施層 (Infrastructure)**: 提供技術實現細節
- **介面層 (Interfaces)**: 處理輸入和輸出（API, CLI, WebUI）

## 錯誤處理

應用程式使用結構化的錯誤處理系統，所有錯誤都實現了 `Error` trait，並提供了豐富的上下文信息，便於調試和排除故障。

## 貢獻

歡迎提交 Pull Request 或開 Issue 反饋問題。

## 授權

MIT

## 未來計劃

- 添加對更多 DNS 提供商的支援
- 實現 Docker 化部署
- 改進 Web UI 介面
- 添加監控和統計功能
