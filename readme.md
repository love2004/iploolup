# Rust DDNS Updater

Rust DDNS Updater 是一個用於自動更新 Cloudflare DNS 記錄的工具，支持 IPv4 和 IPv6 地址。

## 功能特性

- 自動檢測公共 IP 地址變化並更新 Cloudflare DNS 記錄
- 支持 IPv4 和 IPv6 地址
- 支持多種配置方式（環境變量、配置文件）
- 提供 Web 介面查看更新狀態
- 提供 REST API 進行遠程控制

## 項目結構

項目使用領域驅動設計 (DDD) 風格的架構：

```
src/
├── domain/        - 領域層：核心業務邏輯和實體
│   ├── config/    - 配置實體和驗證
│   ├── dns/       - DNS 記錄和服務接口
│   ├── http/      - HTTP 客戶端接口
│   ├── ip/        - IP 地址服務接口
│   └── error.rs   - 領域錯誤定義
├── application/   - 應用層：業務邏輯協調和流程
│   └── ddns/      - DDNS 服務核心邏輯
├── infrastructure/ - 基礎設施層：外部服務集成
│   ├── dns/       - DNS 提供商實現
│   ├── http/      - HTTP 客戶端實現
│   └── ip/        - IP 地址服務實現
├── interfaces/    - 接口層：API 和 Web 界面
│   ├── api/       - REST API 處理程序
│   └── web/       - Web 界面
├── lib.rs         - 庫導出
└── main.rs        - 應用程序入口點
```

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

```bash
# 構建鏡像
docker build -t rust-ddns-updater .

# 運行容器
docker run -d -p 8080:8080 \
  -e DDNS_API_TOKEN=your_cloudflare_token \
  -e DDNS_ZONE_ID=your_zone_id \
  -e DDNS_RECORD_ID=your_record_id \
  -e DDNS_RECORD_NAME=your.domain.com \
  --name ddns-updater \
  rust-ddns-updater
```

## 配置

配置可以通過配置文件或環境變量提供。

### 配置文件

默認配置文件位於 `config/ddns.json`：

```json
{
  "api_token": "your_cloudflare_api_token",
  "zone_id": "your_cloudflare_zone_id",
  "record_id": "your_cloudflare_record_id",
  "record_name": "your.domain.com",
  "update_interval": 300,
  "ip_type": "ipv4",
  "log": {
    "level": "info"
  }
}
```

### 環境變量

- `DDNS_API_TOKEN` - Cloudflare API 令牌
- `DDNS_ZONE_ID` - Cloudflare 區域 ID
- `DDNS_RECORD_ID` - DNS 記錄 ID
- `DDNS_RECORD_NAME` - DNS 記錄名稱
- `DDNS_UPDATE_INTERVAL` - 更新間隔（秒）
- `DDNS_IP_TYPE` - IP 類型 (`ipv4` 或 `ipv6`)
- `DDNS_LOG_LEVEL` - 日誌級別

## 使用方法

### 命令行

```bash
# 啟動 DDNS 服務
cloudflare-ddns

# 啟動 Web 服務器
cloudflare-ddns --web

# 指定配置文件
CONFIG_FILE=/path/to/config.json cloudflare-ddns
```

### Web 介面

啟動 Web 服務器後，訪問 http://localhost:8080 查看狀態介面。

### API 端點

- `GET /api/status` - 獲取 DDNS 服務狀態
- `POST /api/update` - 強制更新 DNS 記錄
- `POST /api/restart` - 重啟 DDNS 服務

## 開發

### 運行測試

```bash
cargo test
```

### 運行代碼檢查

```bash
cargo clippy
```

## 貢獻

歡迎提交 Pull Request 或提出 Issue。

## 許可證

MIT
