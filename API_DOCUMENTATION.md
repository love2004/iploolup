# Rust DDNS Updater API 文檔

本文檔提供了 Rust DDNS Updater 服務提供的所有 API 端點的詳細說明，包括請求和響應格式，以及示例用法。

## 基本信息

- **基礎 URL**: `http://localhost:8080` (默認)
- **格式**: 所有 API 端點都返回 JSON 格式的響應
- **錯誤處理**: API 錯誤會返回適當的 HTTP 狀態碼和包含錯誤詳情的 JSON 響應

## API 端點

### 1. 健康檢查

檢查服務是否正在運行。

- **URL**: `/api/health`
- **方法**: `GET`
- **響應**:
  ```json
  {
    "status": "ok"
  }
  ```
- **狀態碼**:
  - `200 OK`: 服務正常運行
  - `500 Internal Server Error`: 服務出現錯誤

### 2. 服務狀態

獲取 DDNS 服務的當前運行狀態。

- **URL**: `/api/status`
- **方法**: `GET`
- **響應**:
  ```json
  {
    "status": "running",
    "version": "0.1.1",
    "last_update": "2023-03-28 15:30:45 UTC",
    "ip_address": "203.0.113.42",
    "domain": "example.com"
  }
  ```
- **響應字段**:
  - `status`: 服務狀態 (running, stopped)
  - `version`: 軟件版本號
  - `last_update`: 最後更新 DNS 記錄的時間 (可選)
  - `ip_address`: 當前的 IP 地址 (可選)
  - `domain`: 當前管理的域名 (可選)
- **狀態碼**:
  - `200 OK`: 成功獲取狀態
  - `500 Internal Server Error`: 服務出現錯誤

### 3. 強制更新 DNS 記錄

強制立即更新 DNS 記錄到當前 IP 地址。

- **URL**: `/api/update`
- **方法**: `POST`
- **請求體**: 無 (空 JSON 對象)
- **響應**:
  ```json
  {
    "success": true,
    "message": "DNS 記錄已更新",
    "updated_records": [
      {
        "name": "example.com",
        "type": "A",
        "content": "203.0.113.42"
      }
    ]
  }
  ```
- **響應字段**:
  - `success`: 是否更新成功
  - `message`: 描述結果的消息
  - `updated_records`: 更新的記錄列表 (可選)
- **狀態碼**:
  - `200 OK`: 請求成功處理 (即使 DNS 記錄未更改)
  - `500 Internal Server Error`: 更新過程出現錯誤

### 4. 重啟 DDNS 服務

重啟 DDNS 更新服務。

- **URL**: `/api/restart`
- **方法**: `POST`
- **請求體**: 無 (空 JSON 對象)
- **響應**:
  ```json
  {
    "success": true,
    "message": "服務已重啟"
  }
  ```
- **響應字段**:
  - `success`: 是否重啟成功
  - `message`: 描述結果的消息
- **狀態碼**:
  - `200 OK`: 服務重啟成功
  - `500 Internal Server Error`: 重啟過程出現錯誤

### 5. 獲取 IP 地址

#### 5.1 獲取 IPv4 地址

獲取當前的公共 IPv4 地址。

- **URL**: `/api/ip/v4`
- **方法**: `GET`
- **響應**:
  ```json
  {
    "ip": "203.0.113.42"
  }
  ```
- **響應字段**:
  - `ip`: 當前的 IPv4 地址
- **狀態碼**:
  - `200 OK`: 成功獲取 IP 地址
  - `404 Not Found`: 無法獲取 IP 地址
  - `500 Internal Server Error`: 獲取過程出現錯誤

#### 5.2 獲取 IPv6 地址

獲取當前的公共 IPv6 地址。

- **URL**: `/api/ip/v6`
- **方法**: `GET`
- **響應**:
  ```json
  {
    "ip": "2001:db8:85a3::8a2e:370:7334"
  }
  ```
- **響應字段**:
  - `ip`: 當前的 IPv6 地址
- **狀態碼**:
  - `200 OK`: 成功獲取 IP 地址
  - `404 Not Found`: 無法獲取 IP 地址
  - `500 Internal Server Error`: 獲取過程出現錯誤

### 6. 配置管理

#### 6.1 獲取配置

獲取當前所有 DDNS 配置。

- **URL**: `/api/config`
- **方法**: `GET`
- **響應**:
  ```json
  {
    "success": true,
    "message": "成功獲取 2 個配置",
    "configs": [
      {
        "api_token": "**********",
        "zone_id": "zone123",
        "record_id": "record456",
        "record_name": "example.com",
        "update_interval": 300,
        "ip_type": "ipv4"
      },
      {
        "api_token": "**********",
        "zone_id": "zone123",
        "record_id": "record789",
        "record_name": "example.com",
        "update_interval": 300,
        "ip_type": "ipv6"
      }
    ]
  }
  ```
- **響應字段**:
  - `success`: 是否獲取成功
  - `message`: 描述結果的消息
  - `configs`: 配置列表
- **狀態碼**:
  - `200 OK`: 成功獲取配置
  - `500 Internal Server Error`: 獲取過程出現錯誤

#### 6.2 保存配置

保存並應用 DDNS 配置。

- **URL**: `/api/config`
- **方法**: `POST`
- **請求體**:
  ```json
  {
    "configs": [
      {
        "api_token": "your_cloudflare_api_token",
        "zone_id": "your_zone_id",
        "record_id": "your_record_id",
        "record_name": "example.com",
        "update_interval": 300,
        "ip_type": "ipv4"
      }
    ]
  }
  ```
- **響應**:
  ```json
  {
    "success": true,
    "message": "成功保存 1 個配置",
    "configs": [
      {
        "api_token": "**********",
        "zone_id": "your_zone_id",
        "record_id": "your_record_id",
        "record_name": "example.com",
        "update_interval": 300,
        "ip_type": "ipv4"
      }
    ]
  }
  ```
- **請求字段**:
  - `configs`: 要保存的配置列表
- **響應字段**:
  - `success`: 是否保存成功
  - `message`: 描述結果的消息
  - `configs`: 已保存的配置列表，API 令牌會被遮蔽
- **狀態碼**:
  - `200 OK`: 配置保存成功
  - `400 Bad Request`: 配置無效
  - `500 Internal Server Error`: 保存過程出現錯誤

#### 6.3 驗證配置

驗證 DDNS 配置有效性。

- **URL**: `/api/config/validate`
- **方法**: `POST`
- **請求體**:
  ```json
  {
    "config": {
      "api_token": "your_cloudflare_api_token",
      "zone_id": "your_zone_id",
      "record_id": "your_record_id",
      "record_name": "example.com",
      "update_interval": 300,
      "ip_type": "ipv4"
    }
  }
  ```
- **響應**:
  ```json
  {
    "success": true,
    "message": "配置驗證通過",
    "is_valid": true
  }
  ```
- **請求字段**:
  - `config`: 要驗證的配置
- **響應字段**:
  - `success`: 驗證請求是否成功處理
  - `message`: 描述結果的消息
  - `is_valid`: 配置是否有效
- **狀態碼**:
  - `200 OK`: 驗證請求處理成功
  - `500 Internal Server Error`: 驗證過程出現錯誤

## 使用示例

### 使用 curl 獲取服務狀態

```bash
curl -X GET http://localhost:8080/api/status
```

### 使用 curl 強制更新 DNS 記錄

```bash
curl -X POST http://localhost:8080/api/update
```

### 使用 curl 獲取當前 IPv4 地址

```bash
curl -X GET http://localhost:8080/api/ip/v4
```

### 使用 curl 保存新配置

```bash
curl -X POST http://localhost:8080/api/config \
  -H "Content-Type: application/json" \
  -d '{
    "configs": [
      {
        "api_token": "your_cloudflare_api_token",
        "zone_id": "your_zone_id",
        "record_id": "your_record_id",
        "record_name": "example.com",
        "update_interval": 300,
        "ip_type": "ipv4"
      }
    ]
  }'
```

## 錯誤響應格式

所有 API 錯誤都會返回包含以下結構的 JSON 響應：

```json
{
  "success": false,
  "message": "錯誤描述"
}
```

- **常見錯誤狀態碼**:
  - `400 Bad Request`: 請求格式錯誤或參數無效
  - `404 Not Found`: 請求的資源不存在
  - `500 Internal Server Error`: 服務器內部錯誤

## 配置欄位說明

- `api_token`: Cloudflare API 令牌
- `zone_id`: Cloudflare 區域 ID
- `record_id`: DNS 記錄 ID
- `record_name`: DNS 記錄名稱 (域名)
- `update_interval`: 更新間隔，單位為秒
- `ip_type`: IP 類型，可以是 "ipv4" 或 "ipv6" 