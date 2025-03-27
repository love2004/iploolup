/**
 * UI 交互模塊
 * 處理 UI 元素的顯示和隱藏
 */

// 全局數據
let currentZoneId = "";
let ddnsRecords = [];
let allRecords = [];

// 顯示加載狀態
function showLoading(id, show) {
    const element = document.getElementById(id);
    if (element) {
        element.style.display = show ? "block" : "none";
    }
}

// 顯示錯誤信息
function showError(id, message) {
    const element = document.getElementById(id);
    if (element) {
        element.textContent = message;
        element.style.display = "block";
    }
}

// 顯示成功信息
function showSuccess(id, message) {
    const element = document.getElementById(id);
    if (element) {
        element.textContent = message;
        element.style.display = "block";
    }
}

// 隱藏元素
function hideElement(id) {
    const element = document.getElementById(id);
    if (element) {
        element.style.display = "none";
    }
}

// 顯示元素
function showElement(id) {
    const element = document.getElementById(id);
    if (element) {
        element.style.display = "block";
    }
}

// 填充區域選擇框
function populateZoneSelect(zones) {
    const select = document.getElementById("zone-select");
    if (!select) return;
    
    select.innerHTML = '<option value="">-- 選擇區域 --</option>';
    
    zones.forEach(zone => {
        const option = document.createElement("option");
        option.value = zone.id;
        option.textContent = zone.name;
        select.appendChild(option);
    });
}

// 填充記錄表格
function populateRecordsTable(records) {
    const tbody = document.getElementById("records-body");
    if (!tbody) return;
    
    tbody.innerHTML = "";
    
    if (records.length === 0) {
        const tr = document.createElement("tr");
        tr.innerHTML = `<td colspan="6" style="text-align: center;">沒有記錄</td>`;
        tbody.appendChild(tr);
        return;
    }
    
    records.forEach(record => {
        // 只顯示 A 和 AAAA 記錄
        if (record.type !== "A" && record.type !== "AAAA") return;
        
        const tr = document.createElement("tr");
        
        // 檢查記錄是否在DDNS配置中
        const isDdns = ddnsRecords.some(ddns => ddns.record_id === record.id);
        const isAppManaged = isDdns || (record.comment && record.comment.includes("Managed by DDNS Updater"));
        
        // 設置行樣式
        if (isDdns) {
            tr.style.backgroundColor = "#e8f8f5";
        } else if (isAppManaged) {
            tr.style.backgroundColor = "#ebf5fb";
        }
        
        tr.innerHTML = `
            <td>${record.name}</td>
            <td>${record.type}</td>
            <td>${record.content}</td>
            <td>${record.ttl === 1 ? "自動" : record.ttl}</td>
            <td>${record.proxied ? "是" : "否"}</td>
            <td>
                ${isDdns ? '<span class="badge badge-ddns">DDNS</span>' : ''}
                ${isAppManaged && !isDdns ? '<span class="badge badge-app">APP</span>' : ''}
                ${!isAppManaged ? '<span class="badge badge-other">其他</span>' : ''}
                <button onclick="editRecord('${record.id}')" class="update">編輯</button>
                <button onclick="deleteRecord('${record.id}')" class="delete">刪除</button>
                ${isDdns ? `<button onclick="updateDdnsNow('${record.id}')" class="update">立即更新</button>` : ''}
            </td>
        `;
        
        tbody.appendChild(tr);
    });
}

// 填充DDNS表格
function populateDdnsTable(configs) {
    const tbody = document.getElementById("ddns-body");
    if (!tbody) return;
    
    tbody.innerHTML = "";
    
    if (configs.length === 0) {
        const tr = document.createElement("tr");
        tr.innerHTML = `<td colspan="7" style="text-align: center;">沒有 DDNS 配置</td>`;
        tbody.appendChild(tr);
        return;
    }
    
    configs.forEach(config => {
        const record = allRecords.find(r => r.id === config.record_id);
        const recordName = record ? record.name : config.record_name;
        const recordType = config.ip_type === "ipv4" ? "A" : "AAAA";
        
        const tr = document.createElement("tr");
        tr.innerHTML = `
            <td>${recordName}</td>
            <td>${recordType}</td>
            <td>${config.ip_type}</td>
            <td>${config.update_interval}秒</td>
            <td>${config.last_update ? new Date(config.last_update).toLocaleString() : "從未"}</td>
            <td>${config.current_ip || "未知"}</td>
            <td>
                <button onclick="updateDdnsNow('${config.record_id}')" class="update">立即更新</button>
                <button onclick="editDdnsConfig('${config.record_id}')" class="update">編輯</button>
                <button onclick="deleteDdnsConfig('${config.record_id}')" class="delete">停用DDNS</button>
            </td>
        `;
        
        tbody.appendChild(tr);
    });
}

// 重置表單
function resetForm() {
    const nameInput = document.getElementById("new-record-name");
    const contentInput = document.getElementById("new-record-content");
    const ttlInput = document.getElementById("new-record-ttl");
    const proxiedCheckbox = document.getElementById("new-record-proxied");
    const enableDdnsCheckbox = document.getElementById("enable-ddns");
    const updateIntervalInput = document.getElementById("ddns-update-interval");
    const recordIdInput = document.getElementById("editing-record-id");
    const zoneIdInput = document.getElementById("editing-zone-id");
    
    if (nameInput) nameInput.value = "";
    if (contentInput) contentInput.value = "";
    if (ttlInput) ttlInput.value = "120";
    if (proxiedCheckbox) proxiedCheckbox.checked = false;
    if (enableDdnsCheckbox) enableDdnsCheckbox.checked = false;
    if (updateIntervalInput) updateIntervalInput.value = "300";
    if (recordIdInput) recordIdInput.value = "";
    if (zoneIdInput) zoneIdInput.value = "";
    
    hideElement("ddns-settings");
    hideElement("update-record-btn");
    hideElement("cancel-edit-btn");
    showElement("create-record-btn");
    hideElement("create-error");
    hideElement("create-success");
}

// 載入DDNS配置
async function loadDdnsConfigs() {
    try {
        showLoading("ddns-loading", true);
        hideElement("ddns-error");
        
        ddnsRecords = await fetchDdnsConfigs();
        
        populateDdnsTable(ddnsRecords);
        
        // 更新DNS記錄表中的DDNS標記
        if (allRecords.length > 0) {
            populateRecordsTable(allRecords);
        }
    } catch (error) {
        showError("ddns-error", "載入DDNS配置失敗: " + error.message);
    } finally {
        showLoading("ddns-loading", false);
    }
}

// 編輯記錄
window.editRecord = function(recordId) {
    const record = allRecords.find(r => r.id === recordId);
    if (!record) return;
    
    const nameInput = document.getElementById("new-record-name");
    const typeSelect = document.getElementById("new-record-type");
    const contentInput = document.getElementById("new-record-content");
    const ttlInput = document.getElementById("new-record-ttl");
    const proxiedCheckbox = document.getElementById("new-record-proxied");
    const recordIdInput = document.getElementById("editing-record-id");
    const zoneIdInput = document.getElementById("editing-zone-id");
    
    if (nameInput) nameInput.value = record.name;
    if (typeSelect) typeSelect.value = record.type;
    if (contentInput) contentInput.value = record.content;
    if (ttlInput) ttlInput.value = record.ttl === 1 ? 120 : record.ttl;
    if (proxiedCheckbox) proxiedCheckbox.checked = record.proxied;
    if (recordIdInput) recordIdInput.value = recordId;
    if (zoneIdInput) zoneIdInput.value = currentZoneId;
    
    // 檢查是否有DDNS配置
    const ddnsConfig = ddnsRecords.find(config => config.record_id === recordId);
    const enableDdnsCheckbox = document.getElementById("enable-ddns");
    const updateIntervalInput = document.getElementById("ddns-update-interval");
    
    if (enableDdnsCheckbox) {
        enableDdnsCheckbox.checked = !!ddnsConfig;
    }
    
    if (ddnsConfig && updateIntervalInput) {
        updateIntervalInput.value = ddnsConfig.update_interval;
        showElement("ddns-settings");
    } else {
        hideElement("ddns-settings");
    }
    
    hideElement("create-record-btn");
    showElement("update-record-btn");
    showElement("cancel-edit-btn");
};

// 刪除記錄
window.deleteRecord = async function(recordId) {
    if (!confirm("確定要刪除此記錄嗎?")) return;
    
    try {
        await deleteDnsRecord(currentZoneId, recordId);
        
        // 檢查是否有DDNS配置
        const ddnsConfig = ddnsRecords.find(config => config.record_id === recordId);
        if (ddnsConfig) {
            await deleteDdnsConfig(recordId);
        }
        
        // 重新載入記錄
        const records = await fetchDnsRecords(currentZoneId);
        allRecords = records;
        populateRecordsTable(records);
        
        // 重新載入DDNS配置
        await loadDdnsConfigs();
    } catch (error) {
        alert("刪除記錄失敗: " + error.message);
    }
};

// 編輯DDNS配置
window.editDdnsConfig = function(recordId) {
    const record = allRecords.find(r => r.id === recordId);
    if (!record) return;
    
    const ddnsConfig = ddnsRecords.find(config => config.record_id === recordId);
    if (!ddnsConfig) return;
    
    editRecord(recordId);
};

// 刪除DDNS配置
window.deleteDdnsConfig = async function(recordId) {
    if (!confirm("確定要停用此DDNS配置嗎?")) return;
    
    try {
        await deleteDdnsConfig(recordId);
        
        // 重新載入DDNS配置
        await loadDdnsConfigs();
        
        // 重新載入記錄
        const records = await fetchDnsRecords(currentZoneId);
        allRecords = records;
        populateRecordsTable(records);
    } catch (error) {
        alert("停用DDNS配置失敗: " + error.message);
    }
}; 