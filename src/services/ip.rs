use crate::error::AppError;

/// 獲取當前 IPv4 地址
/// 
/// # 返回
/// 
/// - `Result<String, AppError>`: 成功時返回 IPv4 地址，失敗時返回錯誤
/// 
/// # 錯誤
/// 
/// 當以下情況發生時返回錯誤：
/// - API 請求失敗
/// - 響應解析失敗
pub async fn fetch_ipv4() -> Result<String, AppError> {
    fetch_ip("https://api4.ipify.org").await
}

/// 獲取當前 IPv6 地址
/// 
/// # 返回
/// 
/// - `Result<String, AppError>`: 成功時返回 IPv6 地址，失敗時返回錯誤
/// 
/// # 錯誤
/// 
/// 當以下情況發生時返回錯誤：
/// - API 請求失敗
/// - 響應解析失敗
pub async fn fetch_ipv6() -> Result<String, AppError> {
    fetch_ip("https://api6.ipify.org").await
}

/// 從指定 URL 獲取 IP 地址
/// 
/// # 參數
/// 
/// - `url`: IP 查詢服務的 URL
/// 
/// # 返回
/// 
/// - `Result<String, AppError>`: 成功時返回 IP 地址，失敗時返回錯誤
/// 
/// # 錯誤
/// 
/// 當以下情況發生時返回錯誤：
/// - API 請求失敗
/// - 響應解析失敗
async fn fetch_ip(url: &str) -> Result<String, AppError> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;
    
    let ip = response.text()
        .await
        .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;
    
    Ok(ip)
} 