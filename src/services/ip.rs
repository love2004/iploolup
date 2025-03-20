use crate::error::AppError;

pub async fn fetch_ipv4() -> Result<String, AppError> {
    fetch_ip("https://api4.ipify.org").await
}

pub async fn fetch_ipv6() -> Result<String, AppError> {
    fetch_ip("https://api6.ipify.org").await
}

async fn fetch_ip(url: &str) -> Result<String, AppError> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;
    
    let ip = response.text()
        .await
        .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;
    
    Ok(ip)
} 