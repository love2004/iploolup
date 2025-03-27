/**
 * API 交互模塊
 * 處理與後端API的通信
 */

// 存儲和獲取 API token
function setApiToken(token) {
    localStorage.setItem('cloudflare_api_token', token);
}

function getApiToken() {
    return localStorage.getItem('cloudflare_api_token');
}

/**
 * Cloudflare API
 */

// 獲取區域列表
async function fetchZones() {
    const token = getApiToken();
    if (!token) throw new Error("缺少 API 令牌");
    
    const response = await fetch('/api/zones', {
        method: 'GET',
        headers: {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json'
        }
    });
    
    if (!response.ok) {
        const error = await response.json().catch(() => null);
        throw new Error(error?.message || "獲取區域失敗");
    }
    
    return response.json();
}

// 獲取 DNS 記錄
async function fetchDnsRecords(zoneId) {
    const token = getApiToken();
    if (!token) throw new Error("缺少 API 令牌");
    
    const response = await fetch(`/api/zones/${zoneId}/dns_records?type=A,AAAA`, {
        method: 'GET',
        headers: {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json'
        }
    });
    
    if (!response.ok) {
        const error = await response.json().catch(() => null);
        throw new Error(error?.message || "獲取DNS記錄失敗");
    }
    
    return response.json();
}

// 創建 DNS 記錄
async function createDnsRecord(zoneId, record) {
    const token = getApiToken();
    if (!token) throw new Error("缺少 API 令牌");
    
    const response = await fetch(`/api/zones/${zoneId}/dns_records`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(record)
    });
    
    if (!response.ok) {
        const error = await response.json().catch(() => null);
        throw new Error(error?.message || "創建DNS記錄失敗");
    }
    
    return response.json();
}

// 更新 DNS 記錄
async function updateDnsRecord(zoneId, recordId, record) {
    const token = getApiToken();
    if (!token) throw new Error("缺少 API 令牌");
    
    const response = await fetch(`/api/zones/${zoneId}/dns_records/${recordId}`, {
        method: 'PUT',
        headers: {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(record)
    });
    
    if (!response.ok) {
        const error = await response.json().catch(() => null);
        throw new Error(error?.message || "更新DNS記錄失敗");
    }
    
    return response.json();
}

// 刪除 DNS 記錄
async function deleteDnsRecord(zoneId, recordId) {
    const token = getApiToken();
    if (!token) throw new Error("缺少 API 令牌");
    
    const response = await fetch(`/api/zones/${zoneId}/dns_records/${recordId}`, {
        method: 'DELETE',
        headers: {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json'
        }
    });
    
    if (!response.ok) {
        const error = await response.json().catch(() => null);
        throw new Error(error?.message || "刪除DNS記錄失敗");
    }
    
    return true;
}

/**
 * 本地 API
 */

// 獲取 DDNS 配置
async function fetchDdnsConfigs() {
    const response = await fetch('/api/ddns/configs', {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        }
    });
    
    if (!response.ok) {
        console.error(`獲取DDNS配置失敗，狀態碼: ${response.status}`);
        try {
            const error = await response.json().catch(() => null);
            throw new Error(error?.message || `獲取DDNS配置失敗，狀態碼: ${response.status}`);
        } catch (e) {
            console.error('解析錯誤響應失敗:', e);
            throw new Error(`獲取DDNS配置失敗，狀態碼: ${response.status}`);
        }
    }
    
    try {
        return await response.json();
    } catch (e) {
        console.error('解析JSON響應失敗:', e);
        throw new Error("獲取DDNS配置失敗: 無效的JSON響應");
    }
}

// 創建 DDNS 配置
async function createDdnsConfig(zoneId, recordId, recordName, ipType, updateInterval) {
    const response = await fetch('/api/ddns/configs', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            zone_id: zoneId,
            record_id: recordId,
            record_name: recordName,
            ip_type: ipType,
            update_interval: updateInterval
        })
    });
    
    if (!response.ok) {
        const error = await response.json().catch(() => null);
        throw new Error(error?.message || "創建DDNS配置失敗");
    }
    
    return response.json();
}

// 更新 DDNS 配置
async function updateDdnsConfig(zoneId, recordId, recordName, ipType, updateInterval) {
    const response = await fetch(`/api/ddns/configs/${recordId}`, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            zone_id: zoneId,
            record_name: recordName,
            ip_type: ipType,
            update_interval: updateInterval
        })
    });
    
    if (!response.ok) {
        const error = await response.json().catch(() => null);
        throw new Error(error?.message || "更新DDNS配置失敗");
    }
    
    return response.json();
}

// 刪除 DDNS 配置
async function deleteDdnsConfig(recordId) {
    const response = await fetch(`/api/ddns/configs/${recordId}`, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        }
    });
    
    if (!response.ok) {
        const error = await response.json().catch(() => null);
        throw new Error(error?.message || "刪除DDNS配置失敗");
    }
    
    return true;
}

// 立即更新 DDNS
async function updateDdnsNow(recordId) {
    const response = await fetch(`/api/ddns/configs/${recordId}/update`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        }
    });
    
    if (!response.ok) {
        const error = await response.json().catch(() => null);
        throw new Error(error?.message || "更新DDNS失敗");
    }
    
    return response.json();
}

// 獲取當前 IP
async function fetchCurrentIp(type) {
    const response = await fetch(`/api/ip/${type}`, {
        method: 'GET'
    });
    
    if (!response.ok) {
        const error = await response.json().catch(() => null);
        throw new Error(error?.message || "獲取當前IP失敗");
    }
    
    return response.json();
} 