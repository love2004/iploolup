# Rust DDNS Updater

Rust DDNS Updater 是一個用於自動更新 Cloudflare DNS 記錄的工具，支持 IPv4 和 IPv6 地址。專為長期運行和穩定性設計，具有完整的事件系統和並發安全機制。

## 功能特性

- 自動檢測公共 IP 地址變化並更新 Cloudflare DNS 記錄
- 支持 IPv4 和 IPv6 地址
- 支持多種配置方式（環境變量、配置文件）
- 提供 Web 介面查看更新狀態和控制 DDNS 服務
- 提供 REST API 進行遠程控制
- 事件驅動系統用於模組間通信
- 安全的異步並發處理

## 項目架構

項目使用領域驅動設計 (DDD) 風格的架構：

```
src/
├── domain/        - 領域層：核心業務邏輯和實體
│   ├── config/    - 配置實體和驗證
│   ├── dns/       - DNS 記錄和服務接口
│   ├── error.rs   - 領域錯誤定義
│   ├── http/      - HTTP 客戶端接口
│   ├── ip/        - IP 地址服務接口
│   └── state/     - 狀態管理接口
├── application/   - 應用層：業務邏輯協調和流程
│   ├── config/    - 應用配置管理
│   ├── ddns/      - DDNS 服務核心邏輯
│   ├── error.rs   - 應用層錯誤定義
│   ├── events/    - 事件系統
│   └── factories/ - 服務工廠模式
├── infrastructure/ - 基礎設施層：外部服務集成
│   ├── dns/       - DNS 提供商實現
│   ├── http/      - HTTP 客戶端實現
│   ├── ip/        - IP 地址服務實現
│   └── state/     - 狀態存儲實現
├── interfaces/    - 接口層：API 和 Web 界面
│   ├── api/       - REST API 處理程序
│   └── web/       - Web 界面
├── lib.rs         - 庫導出
└── main.rs        - 應用程序入口點
```

## 特殊設計特性

### 事件系統

專案實現了完整的事件發布/訂閱系統，用於解耦各模組間的通信：

- 支援的事件類型：`RestartDdnsService`, `ForceUpdateDns`, `ConfigChanged`
- 異步事件處理，確保高效和非阻塞操作
- 基於 tokio 的廣播通道實現

### 並發安全

- 使用 `tokio::sync::Mutex` 和 `tokio::sync::RwLock` 確保異步上下文中的安全
- 適當的鎖作用域管理，避免跨 `.await` 點持有鎖
- 安全的資源獲取和釋放模式

### 服務生命週期管理

- 使用服務工廠模式創建和管理服務實例
- 支援熱重啟服務而不中斷應用程序
- 優雅的啟動和關閉流程

## 安裝

### 從源代碼構建

```bash
# 克隆倉庫
git clone https://github.com/yourusername/rust-ddns-updater.git
cd rust-ddns-updater

# 構建
cargo build --release

# 運行
./target/release/cloudflare-ddns
```

### 使用 Docker

#### 使用 Docker Compose (推薦)

```bash
# 克隆倉庫
git clone https://github.com/yourusername/rust-ddns-updater.git
cd rust-ddns-updater

# 編輯環境變量或配置文件
# 可以在 docker-compose.yml 中設置環境變量或使用配置文件

# 啟動容器
docker-compose up -d

# 查看日誌
docker-compose logs -f
```

#### 使用 Docker 命令

```bash
# 構建鏡像
docker build -t rust-ddns-updater .

# 運行容器
docker run -d -p 8080:8080 \
  -v ./config:/app/config \
  -v ./static:/app/static \
  -e CLOUDFLARE_API_TOKEN=your_cloudflare_token \
  -e CLOUDFLARE_ZONE_ID=your_zone_id \
  -e CLOUDFLARE_RECORD_ID=your_record_id \
  -e CLOUDFLARE_RECORD_NAME=your.domain.com \
  --name ddns-updater \
  rust-ddns-updater
```

## 配置

配置可以通過配置文件或環境變量提供。

### 環境變量

#### 基本設定
- `CLOUDFLARE_API_TOKEN` - Cloudflare API 令牌
- `CLOUDFLARE_ZONE_ID` - Cloudflare 區域 ID
- `CLOUDFLARE_RECORD_ID` - DNS 記錄 ID (IPv4)
- `CLOUDFLARE_RECORD_NAME` - DNS 記錄名稱 (IPv4)
- `CLOUDFLARE_RECORD_ID_V6` - IPv6 DNS 記錄 ID (可選)
- `CLOUDFLARE_RECORD_NAME_V6` - IPv6 DNS 記錄名稱 (可選)

#### 進階設定
- `DDNS_UPDATE_INTERVAL` - 更新間隔（秒），預設 300
- `SERVER_HOST` - Web 伺服器主機，預設 0.0.0.0
- `SERVER_PORT` - Web 伺服器端口，預設 8080
- `LOG_LEVEL` - 日誌級別，預設 info

## 使用方法

### 命令行

```bash
# 僅啟動 DDNS 服務
cloudflare-ddns --ddns

# 僅啟動 Web 服務器
cloudflare-ddns --web

# 同時啟動 DDNS 服務和 Web 服務器 (預設)
cloudflare-ddns

# 顯示幫助信息
cloudflare-ddns --help
```

### Web 介面

啟動 Web 服務器後，訪問 http://localhost:8080/ui 查看狀態介面。

### API 端點

- `GET /api/status` - 獲取 DDNS 服務狀態
- `POST /api/update` - 強制更新 DNS 記錄
- `POST /api/restart` - 重啟 DDNS 服務
- `GET /api/ip/v4` - 獲取當前 IPv4 地址
- `GET /api/ip/v6` - 獲取當前 IPv6 地址

## 開發

### 前置需求

- Rust 1.64.0 或更高版本
- 可用的 Cloudflare API 令牌和區域 ID

### 運行測試

```bash
cargo test
```

### 運行代碼檢查

```bash
cargo clippy
```

## 項目狀態

請查看 `rust-ddns-status.md` 和 `implementation_status.md` 檔案以獲取詳細的功能實現狀態。

## 貢獻

歡迎提交 Pull Request 或提出 Issue。

## 許可證

MIT
