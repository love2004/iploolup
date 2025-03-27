use actix_web::dev::ServiceResponse;
use actix_web::http::header::{ContentType, CONTENT_TYPE};
use actix_web::{web, HttpResponse, Responder};
use log::error;

const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="zh-TW">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Cloudflare DDNS 管理界面</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        h1, h2, h3 {
            color: #2c3e50;
        }
        .container {
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
            padding: 20px;
            margin-bottom: 20px;
        }
        input, select, button {
            padding: 8px 12px;
            border: 1px solid #ddd;
            border-radius: 4px;
            margin-bottom: 10px;
        }
        button {
            background-color: #4b72b2;
            color: white;
            cursor: pointer;
            border: none;
            padding: 10px 15px;
        }
        button:hover {
            background-color: #3c5d99;
        }
        .grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
            gap: 15px;
        }
        .card {
            border: 1px solid #e0e0e0;
            border-radius: 4px;
            padding: 15px;
            background-color: #f9f9f9;
        }
        table {
            width: 100%;
            border-collapse: collapse;
        }
        th, td {
            padding: 10px;
            border-bottom: 1px solid #ddd;
            text-align: left;
        }
        th {
            background-color: #f2f2f2;
        }
        .loading {
            display: none;
            margin: 10px 0;
        }
        .error {
            color: #e74c3c;
            margin: 10px 0;
        }
        .success {
            color: #2ecc71;
            margin: 10px 0;
        }
        .section {
            margin-bottom: 30px;
        }
        .hidden {
            display: none;
        }
    </style>
</head>
<body>
    <div class="nav" style="display: flex; gap: 15px; margin-bottom: 20px;">
        <a href="/ui" style="color: #4b72b2; text-decoration: none; font-weight: 500;">首頁</a>
        <a href="/ui/ddns-manager" style="color: #4b72b2; text-decoration: none; font-weight: 500;">DDNS配置管理</a>
    </div>
    
    <h1>Cloudflare DDNS 管理界面</h1>

    <div class="container section">
        <h2>API 認證</h2>
        <div>
            <label for="api-token">Cloudflare API Token:</label>
            <input type="password" id="api-token" placeholder="API Token" style="width: 300px;">
            <button id="connect-btn">連接</button>
            <div id="connect-loading" class="loading">連接中...</div>
            <div id="connect-error" class="error"></div>
            <div id="connect-success" class="success"></div>
        </div>
    </div>

    <div id="management-section" class="hidden">
        <div class="container section">
            <h2>區域 (Zones) 管理</h2>
            <div>
                <select id="zone-select" style="min-width: 300px;">
                    <option value="">-- 選擇區域 --</option>
                </select>
                <button id="load-records-btn">載入DNS記錄</button>
                <div id="records-loading" class="loading">載入中...</div>
                <div id="records-error" class="error"></div>
            </div>
        </div>

        <div class="container section">
            <h2>DNS 記錄</h2>
            <table id="records-table">
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>名稱</th>
                        <th>類型</th>
                        <th>內容</th>
                        <th>TTL</th>
                        <th>代理狀態</th>
                    </tr>
                </thead>
                <tbody id="records-body">
                    <!-- 記錄將在這裡動態添加 -->
                </tbody>
            </table>
        </div>

        <div class="container section">
            <h2>創建 DDNS 記錄</h2>
            <div>
                <div>
                    <label for="new-record-name">記錄名稱:</label>
                    <input type="text" id="new-record-name" placeholder="例如: ddns.example.com">
                </div>
                <div>
                    <label for="new-record-type">記錄類型:</label>
                    <select id="new-record-type">
                        <option value="A">A (IPv4)</option>
                        <option value="AAAA">AAAA (IPv6)</option>
                    </select>
                </div>
                <div>
                    <label for="new-record-content">IP地址:</label>
                    <input type="text" id="new-record-content" placeholder="自動獲取的IP地址將顯示在這裡" readonly>
                    <button id="fetch-ip-btn">獲取當前IP</button>
                </div>
                <div>
                    <label for="new-record-ttl">TTL (秒):</label>
                    <input type="number" id="new-record-ttl" value="120" min="60">
                </div>
                <div>
                    <label>
                        <input type="checkbox" id="new-record-proxied"> 啟用 Cloudflare 代理
                    </label>
                </div>
                <button id="create-record-btn">創建記錄</button>
                <div id="create-loading" class="loading">創建中...</div>
                <div id="create-error" class="error"></div>
                <div id="create-success" class="success"></div>
            </div>
        </div>
    </div>

    <script>
        // 存儲API令牌
        let apiToken = "";
        
        // 連接按鈕事件
        document.getElementById("connect-btn").addEventListener("click", async () => {
            const tokenInput = document.getElementById("api-token");
            apiToken = tokenInput.value.trim();
            
            if (!apiToken) {
                showError("connect-error", "請輸入API令牌");
                return;
            }
            
            try {
                showLoading("connect-loading", true);
                hideElement("connect-error");
                hideElement("connect-success");
                
                const zones = await fetchZones();
                
                populateZoneSelect(zones);
                
                showLoading("connect-loading", false);
                showSuccess("connect-success", `成功連接！發現 ${zones.length} 個區域。`);
                
                // 顯示管理部分
                document.getElementById("management-section").classList.remove("hidden");
            } catch (error) {
                showLoading("connect-loading", false);
                showError("connect-error", `連接失敗: ${error.message}`);
            }
        });
        
        // 獲取區域列表
        async function fetchZones() {
            const response = await fetch("/api/v1/cloudflare/zones", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({ token: apiToken })
            });
            
            const data = await response.json();
            
            if (!response.ok || data.status === "error") {
                throw new Error(data.message || "未知錯誤");
            }
            
            return data.zones || [];
        }
        
        // 填充區域選擇框
        function populateZoneSelect(zones) {
            const select = document.getElementById("zone-select");
            
            // 清除現有選項（保留第一個）
            while (select.options.length > 1) {
                select.remove(1);
            }
            
            // 添加新區域
            zones.forEach(zone => {
                const option = document.createElement("option");
                option.value = zone.id;
                option.textContent = zone.name;
                select.appendChild(option);
            });
        }
        
        // 載入DNS記錄
        document.getElementById("load-records-btn").addEventListener("click", async () => {
            const zoneSelect = document.getElementById("zone-select");
            const zoneId = zoneSelect.value;
            
            if (!zoneId) {
                showError("records-error", "請選擇一個區域");
                return;
            }
            
            try {
                showLoading("records-loading", true);
                hideElement("records-error");
                
                const records = await fetchDnsRecords(zoneId);
                
                populateRecordsTable(records);
                
                showLoading("records-loading", false);
            } catch (error) {
                showLoading("records-loading", false);
                showError("records-error", `載入記錄失敗: ${error.message}`);
            }
        });
        
        // 獲取DNS記錄
        async function fetchDnsRecords(zoneId) {
            const response = await fetch(`/api/v1/cloudflare/zones/${zoneId}/dns_records`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({ token: apiToken })
            });
            
            const data = await response.json();
            
            if (!response.ok || data.status === "error") {
                throw new Error(data.message || "未知錯誤");
            }
            
            return data.records || [];
        }
        
        // 填充DNS記錄表格
        function populateRecordsTable(records) {
            const tbody = document.getElementById("records-body");
            
            // 清除現有行
            tbody.innerHTML = "";
            
            if (records.length === 0) {
                const row = tbody.insertRow();
                const cell = row.insertCell();
                cell.colSpan = 6;
                cell.textContent = "沒有找到DNS記錄";
                return;
            }
            
            // 添加記錄
            records.forEach(record => {
                const row = tbody.insertRow();
                
                // ID
                const idCell = row.insertCell();
                idCell.textContent = record.id;
                
                // 名稱
                const nameCell = row.insertCell();
                nameCell.textContent = record.name;
                
                // 類型
                const typeCell = row.insertCell();
                typeCell.textContent = record.record_type;
                
                // 內容
                const contentCell = row.insertCell();
                contentCell.textContent = record.content;
                
                // TTL
                const ttlCell = row.insertCell();
                ttlCell.textContent = record.ttl === 1 ? "自動" : record.ttl;
                
                // 代理狀態
                const proxiedCell = row.insertCell();
                proxiedCell.textContent = record.proxied ? "已啟用" : "未啟用";
            });
        }
        
        // 獲取當前IP按鈕
        document.getElementById("fetch-ip-btn").addEventListener("click", async () => {
            const recordTypeSelect = document.getElementById("new-record-type");
            const contentInput = document.getElementById("new-record-content");
            
            const ipType = recordTypeSelect.value === "A" ? "v4" : "v6";
            
            try {
                const response = await fetch(`/api/v1/ip/${ipType}`);
                const data = await response.json();
                
                if (response.ok && data.status === "success") {
                    contentInput.value = data.ip;
                } else {
                    alert(`獲取IP失敗: ${data.message || "未知錯誤"}`);
                }
            } catch (error) {
                alert(`獲取IP失敗: ${error.message}`);
            }
        });
        
        // 創建記錄按鈕
        document.getElementById("create-record-btn").addEventListener("click", async () => {
            const zoneSelect = document.getElementById("zone-select");
            const zoneId = zoneSelect.value;
            
            if (!zoneId) {
                showError("create-error", "請選擇一個區域");
                return;
            }
            
            const nameInput = document.getElementById("new-record-name");
            const typeSelect = document.getElementById("new-record-type");
            const contentInput = document.getElementById("new-record-content");
            const ttlInput = document.getElementById("new-record-ttl");
            const proxiedCheckbox = document.getElementById("new-record-proxied");
            
            const name = nameInput.value.trim();
            const type = typeSelect.value;
            const content = contentInput.value.trim();
            const ttl = parseInt(ttlInput.value, 10);
            const proxied = proxiedCheckbox.checked;
            
            if (!name) {
                showError("create-error", "請輸入記錄名稱");
                return;
            }
            
            if (!content) {
                showError("create-error", "請獲取或輸入IP地址");
                return;
            }
            
            try {
                showLoading("create-loading", true);
                hideElement("create-error");
                hideElement("create-success");
                
                await createDnsRecord(zoneId, name, type, content, ttl, proxied);
                
                showLoading("create-loading", false);
                showSuccess("create-success", "記錄創建成功！");
                
                // 刷新DNS記錄表格
                const records = await fetchDnsRecords(zoneId);
                populateRecordsTable(records);
                
                // 清除表單
                nameInput.value = "";
                contentInput.value = "";
                ttlInput.value = "120";
                proxiedCheckbox.checked = false;
            } catch (error) {
                showLoading("create-loading", false);
                showError("create-error", `創建記錄失敗: ${error.message}`);
            }
        });
        
        // 創建DNS記錄
        async function createDnsRecord(zoneId, name, type, content, ttl, proxied) {
            const requestData = {
                token: apiToken,
                zone_id: zoneId,
                name: name,
                type: type,
                content: content,
                ttl: ttl,
                proxied: proxied
            };
            
            const response = await fetch("/api/v1/cloudflare/dns_records", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify([
                    { token: apiToken },
                    {
                        zone_id: zoneId,
                        name: name,
                        type: type,
                        content: content,
                        ttl: ttl,
                        proxied: proxied
                    }
                ])
            });
            
            const data = await response.json();
            
            if (!response.ok || data.status === "error") {
                throw new Error(data.message || "未知錯誤");
            }
            
            return data.record;
        }
        
        // 顯示錯誤信息
        function showError(elementId, message) {
            const element = document.getElementById(elementId);
            element.textContent = message;
            element.style.display = "block";
        }
        
        // 顯示成功信息
        function showSuccess(elementId, message) {
            const element = document.getElementById(elementId);
            element.textContent = message;
            element.style.display = "block";
        }
        
        // 顯示/隱藏載入中提示
        function showLoading(elementId, show) {
            const element = document.getElementById(elementId);
            element.style.display = show ? "block" : "none";
        }
        
        // 隱藏元素
        function hideElement(elementId) {
            const element = document.getElementById(elementId);
            element.style.display = "none";
        }
    </script>
</body>
</html>"#;

const DDNS_MANAGER_HTML: &str = r#"<!DOCTYPE html>
<html lang="zh-TW">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DDNS 配置管理</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        h1, h2, h3 {
            color: #2c3e50;
        }
        .container {
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
            padding: 20px;
            margin-bottom: 20px;
        }
        input, select, button, textarea {
            padding: 8px 12px;
            border: 1px solid #ddd;
            border-radius: 4px;
            margin-bottom: 10px;
        }
        button {
            background-color: #4b72b2;
            color: white;
            cursor: pointer;
            border: none;
            padding: 10px 15px;
        }
        button:hover {
            background-color: #3c5d99;
        }
        button.delete {
            background-color: #e74c3c;
        }
        button.delete:hover {
            background-color: #c0392b;
        }
        .grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
            gap: 15px;
        }
        .card {
            border: 1px solid #e0e0e0;
            border-radius: 4px;
            padding: 15px;
            background-color: #f9f9f9;
            margin-bottom: 15px;
        }
        table {
            width: 100%;
            border-collapse: collapse;
        }
        th, td {
            padding: 10px;
            border-bottom: 1px solid #ddd;
            text-align: left;
        }
        th {
            background-color: #f2f2f2;
        }
        .loading {
            display: none;
            margin: 10px 0;
        }
        .error {
            color: #e74c3c;
            margin: 10px 0;
        }
        .success {
            color: #2ecc71;
            margin: 10px 0;
        }
        .section {
            margin-bottom: 30px;
        }
        .hidden {
            display: none;
        }
        .config-form {
            margin-top: 20px;
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 10px;
        }
        .form-group {
            margin-bottom: 15px;
        }
        .form-group label {
            display: block;
            margin-bottom: 5px;
            font-weight: 500;
        }
        .full-width {
            grid-column: 1 / -1;
        }
        .nav {
            display: flex;
            gap: 15px;
            margin-bottom: 20px;
        }
        .nav a {
            color: #4b72b2;
            text-decoration: none;
            font-weight: 500;
        }
        .nav a:hover {
            text-decoration: underline;
        }
        .action-buttons {
            display: flex;
            gap: 10px;
            margin-top: 15px;
        }
    </style>
</head>
<body>
    <div class="nav">
        <a href="/ui">首頁</a>
        <a href="/ui/ddns-manager">DDNS配置管理</a>
    </div>

    <h1>DDNS 配置管理</h1>

    <div class="container section">
        <h2>當前DDNS配置</h2>
        <div id="configs-loading" class="loading">載入中...</div>
        <div id="configs-error" class="error"></div>
        
        <h3>環境變量配置</h3>
        <div id="env-configs"></div>
        
        <h3>配置文件配置</h3>
        <div id="json-configs"></div>
        
        <div class="action-buttons">
            <button id="refresh-configs-btn">刷新配置</button>
            <button id="restart-service-btn">重啟DDNS服務</button>
        </div>
    </div>

    <div class="container section">
        <h2>新增/編輯配置</h2>
        
        <div class="container">
            <h3>環境變量配置</h3>
            <div class="config-form">
                <div class="form-group">
                    <label for="env-ip-type">IP類型:</label>
                    <select id="env-ip-type">
                        <option value="ipv4">IPv4</option>
                        <option value="ipv6">IPv6</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="env-api-token">API Token:</label>
                    <input type="text" id="env-api-token" placeholder="Cloudflare API Token">
                </div>
                <div class="form-group">
                    <label for="env-zone-id">Zone ID:</label>
                    <input type="text" id="env-zone-id" placeholder="Cloudflare Zone ID">
                </div>
                <div class="form-group">
                    <label for="env-record-id">Record ID:</label>
                    <input type="text" id="env-record-id" placeholder="DNS Record ID">
                </div>
                <div class="form-group">
                    <label for="env-record-name">Record Name:</label>
                    <input type="text" id="env-record-name" placeholder="例如: ddns.example.com">
                </div>
                <div class="form-group">
                    <label for="env-update-interval">更新間隔 (秒):</label>
                    <input type="number" id="env-update-interval" value="300" min="60">
                </div>
            </div>
            <div class="action-buttons">
                <button id="save-env-btn">保存環境變量配置</button>
                <button id="delete-env-btn" class="delete">刪除環境變量配置</button>
            </div>
            <div id="env-save-loading" class="loading">保存中...</div>
            <div id="env-save-error" class="error"></div>
            <div id="env-save-success" class="success"></div>
        </div>
        
        <div class="container" style="margin-top: 20px;">
            <h3>JSON配置文件</h3>
            <div class="config-form">
                <div class="form-group">
                    <label for="json-ip-type">IP類型:</label>
                    <select id="json-ip-type">
                        <option value="ipv4">IPv4</option>
                        <option value="ipv6">IPv6</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="json-api-token">API Token:</label>
                    <input type="text" id="json-api-token" placeholder="Cloudflare API Token">
                </div>
                <div class="form-group">
                    <label for="json-zone-id">Zone ID:</label>
                    <input type="text" id="json-zone-id" placeholder="Cloudflare Zone ID">
                </div>
                <div class="form-group">
                    <label for="json-record-id">Record ID:</label>
                    <input type="text" id="json-record-id" placeholder="DNS Record ID">
                </div>
                <div class="form-group">
                    <label for="json-record-name">Record Name:</label>
                    <input type="text" id="json-record-name" placeholder="例如: ddns.example.com">
                </div>
                <div class="form-group">
                    <label for="json-update-interval">更新間隔 (秒):</label>
                    <input type="number" id="json-update-interval" value="300" min="60">
                </div>
            </div>
            <div class="action-buttons">
                <button id="save-json-btn">保存到配置文件</button>
                <button id="delete-json-btn" class="delete">刪除配置</button>
            </div>
            <div id="json-save-loading" class="loading">保存中...</div>
            <div id="json-save-error" class="error"></div>
            <div id="json-save-success" class="success"></div>
        </div>
    </div>

    <script>
        // 頁面加載時獲取配置
        document.addEventListener('DOMContentLoaded', function() {
            fetchConfigurations();
        });
        
        // 刷新配置按鈕
        document.getElementById('refresh-configs-btn').addEventListener('click', function() {
            fetchConfigurations();
        });
        
        // 獲取所有配置
        async function fetchConfigurations() {
            showLoading('configs-loading', true);
            hideElement('configs-error');
            
            try {
                const response = await fetch('/api/v1/ddns_config');
                const data = await response.json();
                
                if (response.ok && data.status === 'success') {
                    // 渲染環境變量配置
                    renderEnvConfigs(data.env_configs);
                    
                    // 渲染JSON配置
                    renderJsonConfigs(data.configs);
                    
                    showLoading('configs-loading', false);
                } else {
                    throw new Error(data.message || '獲取配置失敗');
                }
            } catch (error) {
                showLoading('configs-loading', false);
                showError('configs-error', `獲取配置失敗: ${error.message}`);
            }
        }
        
        // 渲染環境變量配置
        function renderEnvConfigs(envConfigs) {
            const container = document.getElementById('env-configs');
            container.innerHTML = '';
            
            // IPv4配置
            if (envConfigs['CLOUDFLARE_API_TOKEN'] && 
                envConfigs['CLOUDFLARE_ZONE_ID'] && 
                envConfigs['CLOUDFLARE_RECORD_ID']) {
                const card = document.createElement('div');
                card.className = 'card';
                
                card.innerHTML = `
                    <h4>IPv4 環境變量配置</h4>
                    <p><strong>API Token:</strong> ${maskToken(envConfigs['CLOUDFLARE_API_TOKEN'])}</p>
                    <p><strong>Zone ID:</strong> ${envConfigs['CLOUDFLARE_ZONE_ID']}</p>
                    <p><strong>Record ID:</strong> ${envConfigs['CLOUDFLARE_RECORD_ID']}</p>
                    <p><strong>Record Name:</strong> ${envConfigs['CLOUDFLARE_RECORD_NAME'] || '未設定'}</p>
                    <p><strong>更新間隔:</strong> ${envConfigs['DDNS_UPDATE_INTERVAL'] || '300'} 秒</p>
                    <button class="edit-env-btn" data-type="ipv4">編輯</button>
                `;
                
                container.appendChild(card);
                
                // 填充編輯表單
                card.querySelector('.edit-env-btn').addEventListener('click', function() {
                    document.getElementById('env-ip-type').value = 'ipv4';
                    document.getElementById('env-api-token').value = envConfigs['CLOUDFLARE_API_TOKEN'];
                    document.getElementById('env-zone-id').value = envConfigs['CLOUDFLARE_ZONE_ID'];
                    document.getElementById('env-record-id').value = envConfigs['CLOUDFLARE_RECORD_ID'];
                    document.getElementById('env-record-name').value = envConfigs['CLOUDFLARE_RECORD_NAME'] || '';
                    document.getElementById('env-update-interval').value = envConfigs['DDNS_UPDATE_INTERVAL'] || '300';
                });
            }
            
            // IPv6配置
            if (envConfigs['CLOUDFLARE_API_TOKEN_V6'] || 
                (envConfigs['CLOUDFLARE_API_TOKEN'] && 
                envConfigs['CLOUDFLARE_RECORD_ID_V6'])) {
                const card = document.createElement('div');
                card.className = 'card';
                
                const apiToken = envConfigs['CLOUDFLARE_API_TOKEN_V6'] || envConfigs['CLOUDFLARE_API_TOKEN'];
                const zoneId = envConfigs['CLOUDFLARE_ZONE_ID_V6'] || envConfigs['CLOUDFLARE_ZONE_ID'];
                
                card.innerHTML = `
                    <h4>IPv6 環境變量配置</h4>
                    <p><strong>API Token:</strong> ${maskToken(apiToken)}</p>
                    <p><strong>Zone ID:</strong> ${zoneId}</p>
                    <p><strong>Record ID:</strong> ${envConfigs['CLOUDFLARE_RECORD_ID_V6']}</p>
                    <p><strong>Record Name:</strong> ${envConfigs['CLOUDFLARE_RECORD_NAME_V6'] || '未設定'}</p>
                    <p><strong>更新間隔:</strong> ${envConfigs['DDNS_UPDATE_INTERVAL_V6'] || envConfigs['DDNS_UPDATE_INTERVAL'] || '300'} 秒</p>
                    <button class="edit-env-btn" data-type="ipv6">編輯</button>
                `;
                
                container.appendChild(card);
                
                // 填充編輯表單
                card.querySelector('.edit-env-btn').addEventListener('click', function() {
                    document.getElementById('env-ip-type').value = 'ipv6';
                    document.getElementById('env-api-token').value = apiToken;
                    document.getElementById('env-zone-id').value = zoneId;
                    document.getElementById('env-record-id').value = envConfigs['CLOUDFLARE_RECORD_ID_V6'];
                    document.getElementById('env-record-name').value = envConfigs['CLOUDFLARE_RECORD_NAME_V6'] || '';
                    document.getElementById('env-update-interval').value = envConfigs['DDNS_UPDATE_INTERVAL_V6'] || envConfigs['DDNS_UPDATE_INTERVAL'] || '300';
                });
            }
            
            if (container.children.length === 0) {
                container.innerHTML = '<p>沒有設置環境變量配置</p>';
            }
        }
        
        // 渲染JSON配置
        function renderJsonConfigs(configs) {
            const container = document.getElementById('json-configs');
            container.innerHTML = '';
            
            if (configs && configs.length > 0) {
                configs.forEach(config => {
                    const card = document.createElement('div');
                    card.className = 'card';
                    
                    card.innerHTML = `
                        <h4>${config.ip_type.toUpperCase()} 配置 - ${config.record_name}</h4>
                        <p><strong>API Token:</strong> ${maskToken(config.api_token)}</p>
                        <p><strong>Zone ID:</strong> ${config.zone_id}</p>
                        <p><strong>Record ID:</strong> ${config.record_id}</p>
                        <p><strong>Record Name:</strong> ${config.record_name}</p>
                        <p><strong>更新間隔:</strong> ${config.update_interval} 秒</p>
                        <button class="edit-json-btn" 
                                data-ip-type="${config.ip_type}"
                                data-api-token="${config.api_token}"
                                data-zone-id="${config.zone_id}"
                                data-record-id="${config.record_id}"
                                data-record-name="${config.record_name}"
                                data-update-interval="${config.update_interval}">
                            編輯
                        </button>
                    `;
                    
                    container.appendChild(card);
                    
                    // 填充編輯表單
                    card.querySelector('.edit-json-btn').addEventListener('click', function() {
                        const btn = this;
                        document.getElementById('json-ip-type').value = btn.dataset.ipType;
                        document.getElementById('json-api-token').value = btn.dataset.apiToken;
                        document.getElementById('json-zone-id').value = btn.dataset.zoneId;
                        document.getElementById('json-record-id').value = btn.dataset.recordId;
                        document.getElementById('json-record-name').value = btn.dataset.recordName;
                        document.getElementById('json-update-interval').value = btn.dataset.updateInterval;
                    });
                });
            } else {
                container.innerHTML = '<p>沒有設置JSON配置文件配置</p>';
            }
        }
        
        // 保存環境變量配置
        document.getElementById('save-env-btn').addEventListener('click', async function() {
            const ipType = document.getElementById('env-ip-type').value;
            const apiToken = document.getElementById('env-api-token').value;
            const zoneId = document.getElementById('env-zone-id').value;
            const recordId = document.getElementById('env-record-id').value;
            const recordName = document.getElementById('env-record-name').value;
            const updateInterval = parseInt(document.getElementById('env-update-interval').value, 10);
            
            if (!apiToken || !zoneId || !recordId || !recordName) {
                showError('env-save-error', '請填寫所有必填字段');
                return;
            }
            
            showLoading('env-save-loading', true);
            hideElement('env-save-error');
            hideElement('env-save-success');
            
            try {
                const response = await fetch('/api/v1/ddns_config/save_env', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify([
                        ipType,
                        {
                            cloudflare_api_token: apiToken,
                            cloudflare_zone_id: zoneId,
                            cloudflare_record_id: recordId,
                            cloudflare_record_name: recordName,
                            update_interval: updateInterval,
                            ip_type: ipType
                        }
                    ])
                });
                
                const data = await response.json();
                
                if (response.ok && data.status === 'success') {
                    showLoading('env-save-loading', false);
                    showSuccess('env-save-success', data.message);
                    fetchConfigurations();
                } else {
                    throw new Error(data.message || '保存環境變量配置失敗');
                }
            } catch (error) {
                showLoading('env-save-loading', false);
                showError('env-save-error', `保存失敗: ${error.message}`);
            }
        });
        
        // 刪除環境變量配置
        document.getElementById('delete-env-btn').addEventListener('click', async function() {
            const ipType = document.getElementById('env-ip-type').value;
            
            if (!confirm(`確定要刪除 ${ipType.toUpperCase()} 環境變量配置嗎？`)) {
                return;
            }
            
            showLoading('env-save-loading', true);
            hideElement('env-save-error');
            hideElement('env-save-success');
            
            try {
                const response = await fetch('/api/v1/ddns_config/delete', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(['env', ipType])
                });
                
                const data = await response.json();
                
                if (response.ok && data.status === 'success') {
                    showLoading('env-save-loading', false);
                    showSuccess('env-save-success', data.message);
                    fetchConfigurations();
                } else {
                    throw new Error(data.message || '刪除環境變量配置失敗');
                }
            } catch (error) {
                showLoading('env-save-loading', false);
                showError('env-save-error', `刪除失敗: ${error.message}`);
            }
        });
        
        // 保存JSON配置
        document.getElementById('save-json-btn').addEventListener('click', async function() {
            const ipType = document.getElementById('json-ip-type').value;
            const apiToken = document.getElementById('json-api-token').value;
            const zoneId = document.getElementById('json-zone-id').value;
            const recordId = document.getElementById('json-record-id').value;
            const recordName = document.getElementById('json-record-name').value;
            const updateInterval = parseInt(document.getElementById('json-update-interval').value, 10);
            
            if (!apiToken || !zoneId || !recordId || !recordName) {
                showError('json-save-error', '請填寫所有必填字段');
                return;
            }
            
            showLoading('json-save-loading', true);
            hideElement('json-save-error');
            hideElement('json-save-success');
            
            try {
                const response = await fetch('/api/v1/ddns_config/save', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({
                        api_token: apiToken,
                        zone_id: zoneId,
                        record_id: recordId,
                        record_name: recordName,
                        update_interval: updateInterval,
                        ip_type: ipType
                    })
                });
                
                const data = await response.json();
                
                if (response.ok && data.status === 'success') {
                    showLoading('json-save-loading', false);
                    showSuccess('json-save-success', data.message);
                    fetchConfigurations();
                } else {
                    throw new Error(data.message || '保存JSON配置失敗');
                }
            } catch (error) {
                showLoading('json-save-loading', false);
                showError('json-save-error', `保存失敗: ${error.message}`);
            }
        });
        
        // 刪除JSON配置
        document.getElementById('delete-json-btn').addEventListener('click', async function() {
            const ipType = document.getElementById('json-ip-type').value;
            const recordName = document.getElementById('json-record-name').value;
            
            if (!recordName) {
                showError('json-save-error', '請填寫記錄名稱');
                return;
            }
            
            if (!confirm(`確定要刪除 ${recordName} (${ipType.toUpperCase()}) 配置嗎？`)) {
                return;
            }
            
            showLoading('json-save-loading', true);
            hideElement('json-save-error');
            hideElement('json-save-success');
            
            try {
                const response = await fetch('/api/v1/ddns_config/delete', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify([recordName, ipType])
                });
                
                const data = await response.json();
                
                if (response.ok && data.status === 'success') {
                    showLoading('json-save-loading', false);
                    showSuccess('json-save-success', data.message);
                    fetchConfigurations();
                } else {
                    throw new Error(data.message || '刪除JSON配置失敗');
                }
            } catch (error) {
                showLoading('json-save-loading', false);
                showError('json-save-error', `刪除失敗: ${error.message}`);
            }
        });
        
        // 重啟DDNS服務
        document.getElementById('restart-service-btn').addEventListener('click', async function() {
            if (!confirm('確定要重啟DDNS服務嗎？這將中斷目前的更新進程并重新啟動。')) {
                return;
            }
            
            try {
                const response = await fetch('/api/v1/ddns_config/restart', {
                    method: 'POST'
                });
                
                const data = await response.json();
                
                if (response.ok && data.status === 'success') {
                    alert(data.message);
                } else {
                    throw new Error(data.message || '重啟服務失敗');
                }
            } catch (error) {
                alert(`重啟服務失敗: ${error.message}`);
            }
        });
        
        // 輔助函數
        function maskToken(token) {
            if (!token) return '';
            if (token.length <= 8) return '********';
            return token.substring(0, 4) + '****' + token.substring(token.length - 4);
        }
        
        function showError(elementId, message) {
            const element = document.getElementById(elementId);
            element.textContent = message;
            element.style.display = 'block';
        }
        
        function showSuccess(elementId, message) {
            const element = document.getElementById(elementId);
            element.textContent = message;
            element.style.display = 'block';
        }
        
        function showLoading(elementId, show) {
            const element = document.getElementById(elementId);
            element.style.display = show ? 'block' : 'none';
        }
        
        function hideElement(elementId) {
            const element = document.getElementById(elementId);
            element.style.display = 'none';
        }
    </script>
</body>
</html>
"#;

/// 配置用於 Web UI 的路由
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ui")
            .route("", web::get().to(index))
            .route("/", web::get().to(index))
            .route("/ddns-manager", web::get().to(ddns_manager))
    );
}

/// 提供 Web UI 主頁
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(INDEX_HTML)
}

/// 提供 DDNS 配置管理頁面
async fn ddns_manager() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(DDNS_MANAGER_HTML)
} 