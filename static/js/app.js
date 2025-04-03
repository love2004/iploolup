/**
 * 主應用模塊
 * 處理事件綁定和初始化
 */

// API 基礎路徑
const API_BASE = '/api';  // 恢復 API 前綴

// 在 DOM 載入完成後執行
document.addEventListener("DOMContentLoaded", () => {
    // 檢查是否在 DDNS 管理頁面
    if (document.getElementById("connect-btn")) {
        initDdnsManager();
    }
    initNavigation();
    // 載入初始數據
    loadModuleData('dashboard');
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

// 初始化導航功能
function initNavigation() {
    const navLinks = document.querySelectorAll('.sidebar-nav a');
    const sections = {
        'dashboard': document.getElementById('dashboard-section'),
        'domains': document.getElementById('domains-section'),
        'settings': document.getElementById('settings-section'),
        'logs': document.getElementById('logs-section')
    };

    navLinks.forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            const target = link.getAttribute('href').substring(1);
            
            // 更新活動狀態
            navLinks.forEach(l => l.parentElement.classList.remove('active'));
            link.parentElement.classList.add('active');
            
            // 顯示對應的內容區域
            Object.values(sections).forEach(section => {
                if (section) section.style.display = 'none';
            });
            
            if (sections[target]) {
                sections[target].style.display = 'block';
                // 載入對應模塊的數據
                loadModuleData(target);
            }
        });
    });
}

// 載入模塊數據
async function loadModuleData(module) {
    switch(module) {
        case 'dashboard':
            await loadDashboardData();
            break;
        case 'domains':
            await loadDomainsData();
            break;
        case 'settings':
            await loadSettingsData();
            break;
        case 'logs':
            await loadLogsData();
            break;
    }
}

// 載入儀表板數據
async function loadDashboardData() {
    try {
        const status = await fetch(`${API_BASE}/api/status`).then(r => r.json());
        updateDashboardStatus(status);
    } catch (error) {
        console.error('載入儀表板數據失敗:', error);
    }
}

// 載入域名管理數據
async function loadDomainsData() {
    try {
        const configs = await fetch(`${API_BASE}/api/configs`).then(r => r.json());
        updateDomainsList(configs);
    } catch (error) {
        console.error('載入域名數據失敗:', error);
    }
}

// 載入系統設定數據
async function loadSettingsData() {
    try {
        const configs = await fetch(`${API_BASE}/api/configs`).then(r => r.json());
        updateSettingsForm(configs);
    } catch (error) {
        console.error('載入設定數據失敗:', error);
    }
}

// 載入操作日誌數據
async function loadLogsData() {
    try {
        const logs = await fetch(`${API_BASE}/api/logs`).then(r => r.json());
        updateLogsList(logs);
    } catch (error) {
        console.error('載入日誌數據失敗:', error);
    }
}

// 初始化應用
document.addEventListener('DOMContentLoaded', function() {
    // 初始化應用
    initApp();
    
    // 綁定事件處理器
    bindEventListeners();
    
    // 載入初始數據
    loadInitialData();
});

// 初始化應用
function initApp() {
    console.log('初始化 DDNS 控制平台...');
}

// 綁定事件處理器
function bindEventListeners() {
    // 刷新按鈕
    const refreshBtn = document.getElementById('refresh-btn');
    if (refreshBtn) {
        refreshBtn.addEventListener('click', function() {
            refreshDashboardData();
        });
    }
    
    // 檢查 IP 按鈕
    const checkIpBtn = document.getElementById('check-ip-btn');
    if (checkIpBtn) {
        checkIpBtn.addEventListener('click', function() {
            checkCurrentIp();
        });
    }
    
    // 重啟服務按鈕
    const restartServiceBtn = document.getElementById('restart-service-btn');
    if (restartServiceBtn) {
        restartServiceBtn.addEventListener('click', function() {
            restartDdnsService();
        });
    }
    
    // 添加記錄按鈕
    const addRecordBtn = document.getElementById('add-record-btn');
    if (addRecordBtn) {
        addRecordBtn.addEventListener('click', function() {
            showAddRecordModal();
        });
    }
    
    // 全部更新按鈕
    const updateAllBtn = document.getElementById('update-all-btn');
    if (updateAllBtn) {
        updateAllBtn.addEventListener('click', function() {
            updateAllRecords();
        });
    }
    
    // 對話框關閉按鈕
    const closeBtn = document.querySelector('.close-btn');
    if (closeBtn) {
        closeBtn.addEventListener('click', function() {
            hideModal('record-modal');
        });
    }
    
    // 對話框背景點擊關閉
    const modalBackdrop = document.querySelector('.modal-backdrop');
    if (modalBackdrop) {
        modalBackdrop.addEventListener('click', function() {
            hideModal('record-modal');
        });
    }
    
    // 取消按鈕
    const cancelBtn = document.getElementById('cancel-btn');
    if (cancelBtn) {
        cancelBtn.addEventListener('click', function() {
            hideModal('record-modal');
        });
    }
    
    // 保存按鈕
    const saveBtn = document.getElementById('save-btn');
    if (saveBtn) {
        saveBtn.addEventListener('click', function() {
            saveRecord();
        });
    }
}

// 載入初始數據
async function loadInitialData() {
    try {
        // 顯示載入中的狀態
        showLoading(true);
        
        // 載入狀態數據
        await loadStatusData();
        
        // 載入 DDNS 記錄
        await loadDdnsRecords();
        
        // 載入區域數據 (用於下拉選單)
        await loadZones();
        
        // 隱藏載入狀態
        showLoading(false);
    } catch (error) {
        console.error('載入初始數據失敗:', error);
        showError('載入數據失敗: ' + error.message);
        showLoading(false);
    }
}

// 載入狀態數據
async function loadStatusData() {
    try {
        const response = await fetch('/api/status');
        if (!response.ok) {
            throw new Error('獲取狀態失敗');
        }
        
        const data = await response.json();
        
        // 更新 UI
        const uptimeElement = document.getElementById('uptime');
        if (uptimeElement) {
            uptimeElement.textContent = formatUptime(data.uptime || 0);
        }
        
        const ipv4AddressElement = document.getElementById('ipv4-address');
        if (ipv4AddressElement) {
            ipv4AddressElement.textContent = data.ipv4 || '無';
        }
        
        const ipv6AddressElement = document.getElementById('ipv6-address');
        if (ipv6AddressElement) {
            ipv6AddressElement.textContent = data.ipv6 || '無';
        }
        
        const lastCheckElement = document.getElementById('last-check');
        if (lastCheckElement) {
            lastCheckElement.textContent = formatDateTime(data.last_check);
        }
        
    } catch (error) {
        console.error('載入狀態數據失敗:', error);
        throw error;
    }
}

// 載入 DDNS 記錄
async function loadDdnsRecords() {
    try {
        const response = await fetch('/api/configs');
        if (!response.ok) {
            throw new Error('獲取 DDNS 記錄失敗');
        }
        
        const records = await response.json();
        
        // 清空表格
        const tableBody = document.getElementById('ddns-records');
        if (!tableBody) return;
        
        tableBody.innerHTML = '';
        
        // 如果沒有記錄
        if (records.length === 0) {
            tableBody.innerHTML = `
                <tr>
                    <td colspan="8" class="text-center">沒有找到 DDNS 記錄。點擊 "新增記錄" 按鈕來創建第一條記錄。</td>
                </tr>
            `;
            return;
        }
        
        // 填充表格
        records.forEach(record => {
            const lastUpdateTime = record.last_update_time 
                ? formatDateTime(record.last_update_time)
                : '從未更新';
                
            const statusBadge = getStatusBadge(record.status);
            
            const row = document.createElement('tr');
            row.innerHTML = `
                <td>${record.record_name}</td>
                <td>${record.record_type}</td>
                <td>${record.ip_type}</td>
                <td>${record.current_ip || '未知'}</td>
                <td>${formatInterval(record.update_interval)}</td>
                <td>${lastUpdateTime}</td>
                <td>${statusBadge}</td>
                <td>
                    <button class="btn primary sm update-record" data-id="${record.record_id}">
                        <span class="icon">🔄</span>更新
                    </button>
                    <button class="btn sm edit-record" data-id="${record.record_id}">
                        <span class="icon">✏️</span>編輯
                    </button>
                    <button class="btn danger sm delete-record" data-id="${record.record_id}">
                        <span class="icon">🗑️</span>刪除
                    </button>
                </td>
            `;
            
            tableBody.appendChild(row);
        });
        
        // 綁定記錄操作按鈕事件
        bindRecordButtons();
        
    } catch (error) {
        console.error('載入 DDNS 記錄失敗:', error);
        throw error;
    }
}

// 綁定記錄操作按鈕事件
function bindRecordButtons() {
    // 更新記錄按鈕
    document.querySelectorAll('.update-record').forEach(button => {
        button.addEventListener('click', function() {
            const recordId = this.getAttribute('data-id');
            updateRecord(recordId);
        });
    });
    
    // 編輯記錄按鈕
    document.querySelectorAll('.edit-record').forEach(button => {
        button.addEventListener('click', function() {
            const recordId = this.getAttribute('data-id');
            editRecord(recordId);
        });
    });
    
    // 刪除記錄按鈕
    document.querySelectorAll('.delete-record').forEach(button => {
        button.addEventListener('click', function() {
            const recordId = this.getAttribute('data-id');
            deleteRecord(recordId);
        });
    });
}

// 載入區域數據
async function loadZones() {
    try {
        const response = await fetch('/api/zones');
        if (!response.ok) {
            throw new Error('獲取區域失敗');
        }
        
        const zones = await response.json();
        
        // 清空下拉選單
        const zoneSelect = document.getElementById('zone-id');
        if (!zoneSelect) return;
        
        zoneSelect.innerHTML = '';
        
        // 填充下拉選單
        zones.forEach(zone => {
            const option = document.createElement('option');
            option.value = zone.id;
            option.textContent = zone.name;
            zoneSelect.appendChild(option);
        });
        
    } catch (error) {
        console.error('載入區域數據失敗:', error);
        throw error;
    }
}

// 刷新儀表板數據
async function refreshDashboardData() {
    try {
        showLoading(true);
        await loadStatusData();
        await loadDdnsRecords();
        showLoading(false);
        showSuccess('數據已刷新');
    } catch (error) {
        console.error('刷新數據失敗:', error);
        showError('刷新數據失敗: ' + error.message);
        showLoading(false);
    }
}

// 檢查當前 IP
async function checkCurrentIp() {
    try {
        showLoading(true);
        
        const response = await fetch('/api/ip/check', {
            method: 'POST'
        });
        
        if (!response.ok) {
            throw new Error('檢查 IP 失敗');
        }
        
        const data = await response.json();
        
        // 更新 UI
        const ipv4AddressElement = document.getElementById('ipv4-address');
        if (ipv4AddressElement) {
            ipv4AddressElement.textContent = data.ipv4 || '無';
        }
        
        const ipv6AddressElement = document.getElementById('ipv6-address');
        if (ipv6AddressElement) {
            ipv6AddressElement.textContent = data.ipv6 || '無';
        }
        
        const lastCheckElement = document.getElementById('last-check');
        if (lastCheckElement) {
            lastCheckElement.textContent = formatDateTime(new Date());
        }
        
        showLoading(false);
        showSuccess('IP 檢查完成');
    } catch (error) {
        console.error('檢查 IP 失敗:', error);
        showError('檢查 IP 失敗: ' + error.message);
        showLoading(false);
    }
}

// 重啟 DDNS 服務
async function restartDdnsService() {
    if (!confirm('確定要重啟 DDNS 服務嗎？')) {
        return;
    }
    
    try {
        showLoading(true);
        
        const response = await fetch('/api/restart', {
            method: 'POST'
        });
        
        if (!response.ok) {
            throw new Error('重啟服務失敗');
        }
        
        const data = await response.json();
        
        showLoading(false);
        showSuccess('服務重啟請求已發送: ' + data.message);
        
        // 延遲刷新數據
        setTimeout(refreshDashboardData, 3000);
    } catch (error) {
        console.error('重啟服務失敗:', error);
        showError('重啟服務失敗: ' + error.message);
        showLoading(false);
    }
}

// 更新所有記錄
async function updateAllRecords() {
    try {
        showLoading(true);
        
        const response = await fetch('/api/ddns/update-all', {
            method: 'POST'
        });
        
        if (!response.ok) {
            throw new Error('更新所有記錄失敗');
        }
        
        const data = await response.json();
        
        showLoading(false);
        showSuccess('更新請求已發送: ' + data.message);
        
        // 延遲刷新數據
        setTimeout(loadDdnsRecords, 2000);
    } catch (error) {
        console.error('更新所有記錄失敗:', error);
        showError('更新所有記錄失敗: ' + error.message);
        showLoading(false);
    }
}

// 更新單條記錄
async function updateRecord(recordId) {
    try {
        const response = await fetch(`/api/update`, {
            method: 'POST'
        });
        
        if (!response.ok) {
            throw new Error('更新記錄失敗');
        }
        
        const data = await response.json();
        
        showSuccess('記錄更新成功: ' + data.message);
        
        // 重新載入記錄
        await loadDdnsRecords();
    } catch (error) {
        console.error('更新記錄失敗:', error);
        showError('更新記錄失敗: ' + error.message);
    }
}

// 編輯記錄
async function editRecord(recordId) {
    try {
        // 獲取記錄詳情
        const response = await fetch(`/api/configs`);
        
        if (!response.ok) {
            throw new Error('獲取記錄詳情失敗');
        }
        
        const record = await response.json();
        
        // 填充表單
        document.getElementById('record-id').value = record.record_id;
        document.getElementById('zone-id').value = record.zone_id;
        document.getElementById('record-name').value = record.record_name;
        document.getElementById('ip-type').value = record.ip_type;
        document.getElementById('update-interval').value = record.update_interval;
        
        // 更新對話框標題
        document.getElementById('modal-title').textContent = '編輯 DDNS 記錄';
        
        // 顯示對話框
        showModal('record-modal');
    } catch (error) {
        console.error('編輯記錄失敗:', error);
        showError('編輯記錄失敗: ' + error.message);
    }
}

// 刪除記錄
async function deleteRecord(recordId) {
    if (!confirm('確定要刪除此 DDNS 記錄嗎？此操作無法撤銷。')) {
        return;
    }
    
    try {
        const response = await fetch(`/api/configs`, {
            method: 'DELETE'
        });
        
        if (!response.ok) {
            throw new Error('刪除記錄失敗');
        }
        
        showSuccess('記錄已刪除');
        
        // 重新載入記錄
        await loadDdnsRecords();
    } catch (error) {
        console.error('刪除記錄失敗:', error);
        showError('刪除記錄失敗: ' + error.message);
    }
}

// 顯示添加記錄對話框
function showAddRecordModal() {
    // 重置表單
    document.getElementById('record-form').reset();
    document.getElementById('record-id').value = '';
    
    // 更新對話框標題
    document.getElementById('modal-title').textContent = '添加 DDNS 記錄';
    
    // 顯示對話框
    showModal('record-modal');
}

// 保存記錄
async function saveRecord() {
    // 獲取表單數據
    const recordId = document.getElementById('record-id').value;
    const zoneId = document.getElementById('zone-id').value;
    const recordName = document.getElementById('record-name').value;
    const ipType = document.getElementById('ip-type').value;
    const updateInterval = parseInt(document.getElementById('update-interval').value);
    
    // 基本驗證
    if (!zoneId || !recordName || !ipType || isNaN(updateInterval) || updateInterval < 60) {
        showError('請填寫所有必填欄位，更新間隔必須大於 60 秒');
        return;
    }
    
    try {
        let response;
        
        if (recordId) {
            // 更新現有記錄
            response = await fetch(`/api/configs`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    configs: [{
                        zone_id: zoneId,
                        record_id: recordId,
                        record_name: recordName,
                        ip_type: ipType,
                        update_interval: updateInterval
                    }]
                })
            });
        } else {
            // 創建新記錄
            response = await fetch('/api/configs', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    configs: [{
                        zone_id: zoneId,
                        record_name: recordName,
                        ip_type: ipType,
                        update_interval: updateInterval
                    }]
                })
            });
        }
        
        if (!response.ok) {
            const error = await response.json().catch(() => null);
            throw new Error(error?.message || "保存記錄失敗");
        }
        
        // 隱藏對話框
        hideModal('record-modal');
        
        // 顯示成功訊息
        showSuccess(recordId ? '記錄更新成功' : '記錄創建成功');
        
        // 重新載入記錄
        await loadDdnsRecords();
    } catch (error) {
        console.error('保存記錄失敗:', error);
        showError('保存記錄失敗: ' + error.message);
    }
}

// 工具函數: 格式化運行時間
function formatUptime(seconds) {
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    
    let result = '';
    if (days > 0) result += `${days}天 `;
    if (hours > 0) result += `${hours}小時 `;
    if (minutes > 0) result += `${minutes}分鐘`;
    
    return result.trim() || '剛剛啟動';
}

// 工具函數: 格式化日期時間
function formatDateTime(dateString) {
    if (!dateString) return '未知';
    
    const date = new Date(dateString);
    if (isNaN(date.getTime())) return dateString;
    
    return date.toLocaleString('zh-TW', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit'
    });
}

// 工具函數: 格式化時間間隔
function formatInterval(seconds) {
    if (seconds >= 86400) {
        return `${(seconds / 86400).toFixed(1)}天`;
    } else if (seconds >= 3600) {
        return `${(seconds / 3600).toFixed(1)}小時`;
    } else if (seconds >= 60) {
        return `${(seconds / 60).toFixed(0)}分鐘`;
    } else {
        return `${seconds}秒`;
    }
}

// 工具函數: 獲取狀態標籤
function getStatusBadge(status) {
    if (status === 'running') {
        return `<span class="status-badge success"><span class="icon">✓</span>運行中</span>`;
    } else if (status === 'error') {
        return `<span class="status-badge danger"><span class="icon">✗</span>錯誤</span>`;
    } else if (status === 'updating') {
        return `<span class="status-badge warning"><span class="icon">⟳</span>更新中</span>`;
    } else {
        return `<span class="status-badge">${status || '未知'}</span>`;
    }
}

// 工具函數: 顯示載入中
function showLoading(show) {
    // 創建或移除全局載入指示器
    let loading = document.querySelector('.global-loading');
    
    if (show) {
        if (!loading) {
            loading = document.createElement('div');
            loading.className = 'global-loading';
            loading.innerHTML = `<div class="spinner"></div>`;
            document.body.appendChild(loading);
        }
        // 使用setTimeout確保DOM更新並觸發CSS過渡效果
        setTimeout(() => {
            loading.classList.add('active');
        }, 10);
    } else if (loading) {
        loading.classList.remove('active');
        // 等待過渡效果完成後移除元素
        setTimeout(() => {
            loading.remove();
        }, 300);
    }
}

// 工具函數: 顯示錯誤訊息
function showError(message) {
    const toast = createToast(message, 'error');
    document.body.appendChild(toast);
    
    setTimeout(() => {
        toast.classList.add('show');
        setTimeout(() => {
            toast.classList.remove('show');
            setTimeout(() => toast.remove(), 300);
        }, 3000);
    }, 10);
}

// 工具函數: 顯示成功訊息
function showSuccess(message) {
    const toast = createToast(message, 'success');
    document.body.appendChild(toast);
    
    setTimeout(() => {
        toast.classList.add('show');
        setTimeout(() => {
            toast.classList.remove('show');
            setTimeout(() => toast.remove(), 300);
        }, 3000);
    }, 10);
}

// 工具函數: 創建提示訊息
function createToast(message, type) {
    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.innerHTML = `
        <div class="toast-content">
            <span class="toast-icon">${type === 'success' ? '✓' : '✗'}</span>
            <span class="toast-message">${message}</span>
        </div>
    `;
    return toast;
}

// 工具函數: 顯示對話框
function showModal(modalId) {
    const modal = document.getElementById(modalId);
    if (modal) {
        modal.classList.add('active');
    }
}

// 工具函數: 隱藏對話框
function hideModal(modalId) {
    const modal = document.getElementById(modalId);
    if (modal) {
        modal.classList.remove('active');
    }
} 