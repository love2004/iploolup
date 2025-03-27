/**
 * 主應用模塊
 * 處理事件綁定和初始化
 */

// 在 DOM 載入完成後執行
document.addEventListener("DOMContentLoaded", () => {
    // 檢查是否在 DDNS 管理頁面
    if (document.getElementById("connect-btn")) {
        initDdnsManager();
    }
});

// 初始化 DDNS 管理器
function initDdnsManager() {
    // 連接按鈕事件
    document.getElementById("connect-btn").addEventListener("click", async () => {
        const tokenInput = document.getElementById("api-token");
        const token = tokenInput.value.trim();
        
        if (!token) {
            showError("connect-error", "請輸入API令牌");
            return;
        }
        
        try {
            showLoading("connect-loading", true);
            hideElement("connect-error");
            hideElement("connect-success");
            
            // 設置 API 令牌
            setApiToken(token);
            
            // 獲取區域列表
            const zones = await fetchZones();
            
            populateZoneSelect(zones);
            showElement("management-section");
            showSuccess("connect-success", "連接成功！已獲取 " + zones.length + " 個區域。");
            
            // 載入 DDNS 記錄
            await loadDdnsConfigs();
        } catch (error) {
            showError("connect-error", "連接失敗: " + error.message);
        } finally {
            showLoading("connect-loading", false);
        }
    });
    
    // 載入記錄按鈕事件
    document.getElementById("load-records-btn").addEventListener("click", async () => {
        const zoneSelect = document.getElementById("zone-select");
        const zoneId = zoneSelect.value;
        
        if (!zoneId) {
            showError("records-error", "請選擇一個區域");
            return;
        }
        
        currentZoneId = zoneId;
        
        try {
            showLoading("records-loading", true);
            hideElement("records-error");
            
            const records = await fetchDnsRecords(zoneId);
            allRecords = records;
            
            populateRecordsTable(records);
        } catch (error) {
            showError("records-error", "載入記錄失敗: " + error.message);
        } finally {
            showLoading("records-loading", false);
        }
    });
    
    // 獲取 IP 按鈕事件
    document.getElementById("fetch-ip-btn").addEventListener("click", async () => {
        const typeSelect = document.getElementById("new-record-type");
        const contentInput = document.getElementById("new-record-content");
        const selectedType = typeSelect.value;
        
        try {
            let ip;
            if (selectedType === "A") {
                ip = await fetchCurrentIp("ipv4");
            } else if (selectedType === "AAAA") {
                ip = await fetchCurrentIp("ipv6");
            }
            
            contentInput.value = ip;
        } catch (error) {
            showError("create-error", "獲取IP失敗: " + error.message);
        }
    });
    
    // 創建記錄按鈕事件
    document.getElementById("create-record-btn").addEventListener("click", async () => {
        const zoneSelect = document.getElementById("zone-select");
        const nameInput = document.getElementById("new-record-name");
        const typeSelect = document.getElementById("new-record-type");
        const contentInput = document.getElementById("new-record-content");
        const ttlInput = document.getElementById("new-record-ttl");
        const proxiedCheckbox = document.getElementById("new-record-proxied");
        const enableDdnsCheckbox = document.getElementById("enable-ddns");
        const updateIntervalInput = document.getElementById("ddns-update-interval");
        
        const zoneId = zoneSelect.value;
        const name = nameInput.value.trim();
        const type = typeSelect.value;
        const content = contentInput.value.trim();
        const ttl = parseInt(ttlInput.value, 10);
        const proxied = proxiedCheckbox.checked;
        const enableDdns = enableDdnsCheckbox.checked;
        const updateInterval = parseInt(updateIntervalInput.value, 10);
        
        if (!zoneId) {
            showError("create-error", "請選擇一個區域");
            return;
        }
        
        if (!name) {
            showError("create-error", "請輸入記錄名稱");
            return;
        }
        
        if (!content) {
            showError("create-error", "請輸入IP地址");
            return;
        }
        
        if (isNaN(ttl) || ttl < 60) {
            showError("create-error", "TTL必須至少為60秒");
            return;
        }
        
        if (enableDdns && (isNaN(updateInterval) || updateInterval < 60)) {
            showError("create-error", "更新間隔必須至少為60秒");
            return;
        }
        
        try {
            showLoading("create-loading", true);
            hideElement("create-error");
            hideElement("create-success");
            
            const record = await createDnsRecord(zoneId, {
                type,
                name,
                content,
                ttl,
                proxied
            });
            
            if (enableDdns) {
                await createDdnsConfig(zoneId, record.id, name, type === "A" ? "ipv4" : "ipv6", updateInterval);
            }
            
            showSuccess("create-success", "記錄創建成功！");
            resetForm();
            
            // 重新載入記錄
            const records = await fetchDnsRecords(zoneId);
            allRecords = records;
            populateRecordsTable(records);
            
            // 重新載入DDNS配置
            await loadDdnsConfigs();
        } catch (error) {
            showError("create-error", "創建記錄失敗: " + error.message);
        } finally {
            showLoading("create-loading", false);
        }
    });
    
    // 更新記錄按鈕事件
    document.getElementById("update-record-btn").addEventListener("click", async () => {
        const recordId = document.getElementById("editing-record-id").value;
        const zoneId = document.getElementById("editing-zone-id").value;
        const nameInput = document.getElementById("new-record-name");
        const typeSelect = document.getElementById("new-record-type");
        const contentInput = document.getElementById("new-record-content");
        const ttlInput = document.getElementById("new-record-ttl");
        const proxiedCheckbox = document.getElementById("new-record-proxied");
        const enableDdnsCheckbox = document.getElementById("enable-ddns");
        const updateIntervalInput = document.getElementById("ddns-update-interval");
        
        const name = nameInput.value.trim();
        const type = typeSelect.value;
        const content = contentInput.value.trim();
        const ttl = parseInt(ttlInput.value, 10);
        const proxied = proxiedCheckbox.checked;
        const enableDdns = enableDdnsCheckbox.checked;
        const updateInterval = parseInt(updateIntervalInput.value, 10);
        
        if (!recordId || !zoneId) {
            showError("create-error", "無效的記錄ID或區域ID");
            return;
        }
        
        if (!name) {
            showError("create-error", "請輸入記錄名稱");
            return;
        }
        
        if (!content) {
            showError("create-error", "請輸入IP地址");
            return;
        }
        
        if (isNaN(ttl) || ttl < 60) {
            showError("create-error", "TTL必須至少為60秒");
            return;
        }
        
        if (enableDdns && (isNaN(updateInterval) || updateInterval < 60)) {
            showError("create-error", "更新間隔必須至少為60秒");
            return;
        }
        
        try {
            showLoading("create-loading", true);
            hideElement("create-error");
            hideElement("create-success");
            
            const record = await updateDnsRecord(zoneId, recordId, {
                type,
                name,
                content,
                ttl,
                proxied
            });
            
            // 檢查是否已有DDNS配置
            const existingConfig = ddnsRecords.find(c => c.record_id === recordId);
            
            if (enableDdns) {
                if (existingConfig) {
                    // 更新現有配置
                    await updateDdnsConfig(zoneId, recordId, name, type === "A" ? "ipv4" : "ipv6", updateInterval);
                } else {
                    // 創建新配置
                    await createDdnsConfig(zoneId, recordId, name, type === "A" ? "ipv4" : "ipv6", updateInterval);
                }
            } else if (existingConfig) {
                // 刪除現有配置
                await deleteDdnsConfig(recordId);
            }
            
            showSuccess("create-success", "記錄更新成功！");
            resetForm();
            
            // 重新載入記錄
            const records = await fetchDnsRecords(zoneId);
            allRecords = records;
            populateRecordsTable(records);
            
            // 重新載入DDNS配置
            await loadDdnsConfigs();
        } catch (error) {
            showError("create-error", "更新記錄失敗: " + error.message);
        } finally {
            showLoading("create-loading", false);
        }
    });
    
    // 取消編輯按鈕事件
    document.getElementById("cancel-edit-btn").addEventListener("click", resetForm);
    
    // 刷新DDNS狀態按鈕事件
    document.getElementById("refresh-ddns-btn").addEventListener("click", async () => {
        await loadDdnsConfigs();
    });
    
    // 啟用DDNS復選框事件
    document.getElementById("enable-ddns").addEventListener("change", function() {
        if (this.checked) {
            showElement("ddns-settings");
        } else {
            hideElement("ddns-settings");
        }
    });
    
    // 搜索記錄事件
    document.getElementById("search-records").addEventListener("input", function() {
        const searchTerm = this.value.toLowerCase();
        if (allRecords.length > 0) {
            const filteredRecords = allRecords.filter(record => 
                record.name.toLowerCase().includes(searchTerm) || 
                record.content.toLowerCase().includes(searchTerm) ||
                record.type.toLowerCase().includes(searchTerm)
            );
            populateRecordsTable(filteredRecords);
        }
    });
    
    // 檢查localStorage中是否有保存的token
    const savedToken = getApiToken();
    if (savedToken) {
        document.getElementById("api-token").value = savedToken;
        document.getElementById("connect-btn").click();
    }
    
    // 為全局命名空間添加 updateDdnsNow 函數
    window.updateDdnsNow = async function(recordId) {
        try {
            showLoading("ddns-loading", true);
            hideElement("ddns-error");
            
            await updateDdnsNow(recordId);
            
            // 重新載入DDNS配置
            await loadDdnsConfigs();
        } catch (error) {
            showError("ddns-error", "更新DDNS失敗: " + error.message);
        } finally {
            showLoading("ddns-loading", false);
        }
    };
} 