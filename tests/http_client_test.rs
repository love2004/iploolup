use async_trait::async_trait;
use cloudflare_ddns::domain::error::DomainError;
use cloudflare_ddns::domain::http::HttpClient;
use mockall::predicate::*;
use mockall::mock;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::{json, Value};
use serde::Serialize;

// 創建模擬 HTTP 客戶端
mock! {
    pub HttpClientMock {}
    
    #[async_trait]
    impl HttpClient for HttpClientMock {
        async fn get(&self, url: &str, headers: Option<HeaderMap>) -> Result<String, DomainError>;
        async fn post(&self, url: &str, body: Option<String>, headers: Option<HeaderMap>) -> Result<String, DomainError>;
        async fn put(&self, url: &str, body: Option<String>, headers: Option<HeaderMap>) -> Result<String, DomainError>;
        async fn delete(&self, url: &str, headers: Option<HeaderMap>) -> Result<String, DomainError>;
    }
}

// 擴展模擬客戶端以支持 JSON 方法
impl MockHttpClientMock {
    // 這些方法是手動實現，而不是通過 mockall 自動生成
    pub async fn get_json<T: for<'de> serde::Deserialize<'de> + 'static>(&self, url: &str, headers: Option<HeaderMap>) -> Result<T, DomainError> {
        let response = self.get(url, headers).await?;
        serde_json::from_str(&response).map_err(|e| DomainError::LogicError(format!("JSON 解析錯誤: {}", e)))
    }
    
    pub async fn post_json<T: for<'de> serde::Deserialize<'de> + 'static, U: Serialize + Send + Sync>(&self, url: &str, body: Option<&U>, headers: Option<HeaderMap>) -> Result<T, DomainError> {
        let body_str = match body {
            Some(b) => Some(serde_json::to_string(b).map_err(|e| DomainError::LogicError(format!("JSON 序列化錯誤: {}", e)))?),
            None => None,
        };
        
        let response = self.post(url, body_str, headers).await?;
        serde_json::from_str(&response).map_err(|e| DomainError::LogicError(format!("JSON 解析錯誤: {}", e)))
    }
    
    pub async fn put_json<T: for<'de> serde::Deserialize<'de> + 'static, U: Serialize + Send + Sync>(&self, url: &str, body: Option<&U>, headers: Option<HeaderMap>) -> Result<T, DomainError> {
        let body_str = match body {
            Some(b) => Some(serde_json::to_string(b).map_err(|e| DomainError::LogicError(format!("JSON 序列化錯誤: {}", e)))?),
            None => None,
        };
        
        let response = self.put(url, body_str, headers).await?;
        serde_json::from_str(&response).map_err(|e| DomainError::LogicError(format!("JSON 解析錯誤: {}", e)))
    }
}

#[tokio::test]
async fn test_http_client_get_json() {
    let mut mock = MockHttpClientMock::new();
    
    // 創建一個簡單的響應頭
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
    // 設置 get 方法的預期行為
    mock.expect_get()
        .with(eq("https://api.example.com/test"), eq(Some(headers.clone())))
        .times(1)
        .returning(|_, _| {
            Ok(r#"{"success":true,"data":{"id":"123","name":"test"}}"#.to_string())
        });
    
    // 調用方法並驗證結果
    let result: Value = mock.get_json("https://api.example.com/test", Some(headers))
        .await
        .expect("HTTP request should succeed");
    
    assert!(result["success"].as_bool().unwrap());
    assert_eq!(result["data"]["id"].as_str().unwrap(), "123");
    assert_eq!(result["data"]["name"].as_str().unwrap(), "test");
}

#[tokio::test]
async fn test_http_client_post_json() {
    let mut mock = MockHttpClientMock::new();
    
    // 創建請求和響應標頭
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
    // 創建請求體
    let request_body = json!({
        "name": "test",
        "value": 42
    });
    
    // 設置 post 方法的預期行為
    mock.expect_post()
        .withf(|url, body, _headers| {
            url == "https://api.example.com/test" && 
            body.as_ref().unwrap().contains("test") && 
            body.as_ref().unwrap().contains("42")
        })
        .times(1)
        .returning(|_, _, _| {
            Ok(r#"{"success":true,"id":"new-id-123"}"#.to_string())
        });
    
    // 調用方法並驗證結果
    let result: Value = mock.post_json("https://api.example.com/test", Some(&request_body), Some(headers))
        .await
        .expect("HTTP request should succeed");
    
    assert!(result["success"].as_bool().unwrap());
    assert_eq!(result["id"].as_str().unwrap(), "new-id-123");
}

#[tokio::test]
async fn test_http_client_error_handling() {
    let mut mock = MockHttpClientMock::new();
    
    // 設置 get 方法返回錯誤的預期行為
    mock.expect_get()
        .with(eq("https://api.example.com/error"), eq(None))
        .times(1)
        .returning(|_, _| {
            Err(DomainError::LogicError("Connection error".to_string()))
        });
    
    // 調用方法並驗證錯誤
    let result: Result<Value, DomainError> = mock.get_json("https://api.example.com/error", None).await;
    
    assert!(result.is_err());
    if let Err(DomainError::LogicError(message)) = result {
        assert_eq!(message, "Connection error");
    } else {
        panic!("Expected LogicError");
    }
} 