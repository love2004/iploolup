use actix_web::{web, HttpResponse, Responder, http::StatusCode};
use serde::{Deserialize, Serialize};
use log::{info, error};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use crate::error::AppError;

// 用於API請求的認證模型
#[derive(Deserialize)]
pub struct ApiCredentials {
    token: String,
}

// Zone 列表響應
#[derive(Serialize)]
pub struct CloudflareZone {
    id: String,
    name: String,
}

// DNS記錄響應
#[derive(Serialize)]
pub struct CloudflareDnsRecord {
    id: String,
    name: String,
    #[serde(rename = "type")]
    record_type: String,
    content: String,
    ttl: i64,
    proxied: bool,
}

// 創建DNS記錄的請求模型
#[derive(Deserialize)]
pub struct CreateDnsRecordRequest {
    zone_id: String,
    name: String,
    #[serde(rename = "type")]
    record_type: String,
    content: String,
    ttl: Option<i64>,
    proxied: Option<bool>,
}

// Cloudflare API 響應結構
#[derive(Deserialize, Debug)]
struct CloudflareListResponse<T> {
    success: bool,
    #[serde(default)]
    errors: Vec<serde_json::Value>,
    #[serde(default)]
    messages: Vec<serde_json::Value>,
    result: Option<Vec<T>>,
}

#[derive(Deserialize, Debug)]
struct CloudflareItemResponse<T> {
    success: bool,
    #[serde(default)]
    errors: Vec<serde_json::Value>,
    #[serde(default)]
    messages: Vec<serde_json::Value>,
    result: Option<T>,
}

// Cloudflare Zone結構
#[derive(Deserialize, Debug)]
struct CfZone {
    id: String,
    name: String,
}

// Cloudflare DNS記錄結構
#[derive(Deserialize, Debug)]
struct CfDnsRecord {
    id: String,
    name: String,
    #[serde(rename = "type")]
    record_type: String,
    content: String,
    ttl: i64,
    proxied: bool,
}

// 配置Cloudflare API路由
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cloudflare")
            .route("/zones", web::post().to(list_zones))
            .route("/zones/{zone_id}/dns_records", web::post().to(list_dns_records))
            .route("/dns_records", web::post().to(create_dns_record))
    );
}

// 獲取Cloudflare區域(zones)列表
async fn list_zones(credentials: web::Json<ApiCredentials>) -> impl Responder {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
    match HeaderValue::from_str(&format!("Bearer {}", credentials.token)) {
        Ok(auth_value) => {
            headers.insert(AUTHORIZATION, auth_value);
        },
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "status": "error",
                "message": "Invalid API token format"
            }));
        }
    }
    
    let result = client.get("https://api.cloudflare.com/client/v4/zones?per_page=50")
        .headers(headers)
        .send()
        .await;
        
    match result {
        Ok(response) => {
            match response.json::<CloudflareListResponse<CfZone>>().await {
                Ok(cf_response) => {
                    if cf_response.success {
                        if let Some(zones) = cf_response.result {
                            let zone_list: Vec<CloudflareZone> = zones.into_iter()
                                .map(|zone| CloudflareZone { 
                                    id: zone.id, 
                                    name: zone.name
                                })
                                .collect();
                                
                            HttpResponse::Ok().json(serde_json::json!({
                                "status": "success",
                                "zones": zone_list
                            }))
                        } else {
                            HttpResponse::Ok().json(serde_json::json!({
                                "status": "success",
                                "zones": []
                            }))
                        }
                    } else {
                        let error_msg = format!("Cloudflare API error: {}", 
                            serde_json::to_string(&cf_response.errors).unwrap_or_else(|_| 
                                "Unknown error".to_string()
                            )
                        );
                        
                        error!("{}", error_msg);
                        HttpResponse::BadGateway().json(serde_json::json!({
                            "status": "error",
                            "message": error_msg
                        }))
                    }
                },
                Err(e) => {
                    error!("Failed to parse Cloudflare API response: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "status": "error",
                        "message": format!("Failed to parse API response: {}", e)
                    }))
                }
            }
        },
        Err(e) => {
            error!("Failed to connect to Cloudflare API: {}", e);
            HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to connect to Cloudflare API: {}", e)
            }))
        }
    }
}

// 獲取指定區域的DNS記錄列表
async fn list_dns_records(
    path: web::Path<String>, 
    credentials: web::Json<ApiCredentials>
) -> impl Responder {
    let zone_id = path.into_inner();
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
    match HeaderValue::from_str(&format!("Bearer {}", credentials.token)) {
        Ok(auth_value) => {
            headers.insert(AUTHORIZATION, auth_value);
        },
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "status": "error",
                "message": "Invalid API token format"
            }));
        }
    }
    
    let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records?per_page=100", zone_id);
    let result = client.get(&url)
        .headers(headers)
        .send()
        .await;
        
    match result {
        Ok(response) => {
            match response.json::<CloudflareListResponse<CfDnsRecord>>().await {
                Ok(cf_response) => {
                    if cf_response.success {
                        if let Some(records) = cf_response.result {
                            let record_list: Vec<CloudflareDnsRecord> = records.into_iter()
                                .map(|rec| CloudflareDnsRecord { 
                                    id: rec.id, 
                                    name: rec.name,
                                    record_type: rec.record_type,
                                    content: rec.content,
                                    ttl: rec.ttl,
                                    proxied: rec.proxied
                                })
                                .collect();
                                
                            HttpResponse::Ok().json(serde_json::json!({
                                "status": "success",
                                "records": record_list
                            }))
                        } else {
                            HttpResponse::Ok().json(serde_json::json!({
                                "status": "success",
                                "records": []
                            }))
                        }
                    } else {
                        let error_msg = format!("Cloudflare API error: {}", 
                            serde_json::to_string(&cf_response.errors).unwrap_or_else(|_| 
                                "Unknown error".to_string()
                            )
                        );
                        
                        error!("{}", error_msg);
                        HttpResponse::BadGateway().json(serde_json::json!({
                            "status": "error",
                            "message": error_msg
                        }))
                    }
                },
                Err(e) => {
                    error!("Failed to parse Cloudflare API response: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "status": "error",
                        "message": format!("Failed to parse API response: {}", e)
                    }))
                }
            }
        },
        Err(e) => {
            error!("Failed to connect to Cloudflare API: {}", e);
            HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to connect to Cloudflare API: {}", e)
            }))
        }
    }
}

// 創建新的DNS記錄
async fn create_dns_record(
    credentials: web::Json<(ApiCredentials, CreateDnsRecordRequest)>
) -> impl Responder {
    let (credentials, create_request) = credentials.into_inner();
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
    match HeaderValue::from_str(&format!("Bearer {}", credentials.token)) {
        Ok(auth_value) => {
            headers.insert(AUTHORIZATION, auth_value);
        },
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "status": "error",
                "message": "Invalid API token format"
            }));
        }
    }
    
    // 構建請求體
    let ttl = create_request.ttl.unwrap_or(120);
    let proxied = create_request.proxied.unwrap_or(false);
    
    let request_body = serde_json::json!({
        "type": create_request.record_type,
        "name": create_request.name,
        "content": create_request.content,
        "ttl": ttl,
        "proxied": proxied
    });
    
    let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", create_request.zone_id);
    let result = client.post(&url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await;
        
    match result {
        Ok(response) => {
            match response.json::<CloudflareItemResponse<CfDnsRecord>>().await {
                Ok(cf_response) => {
                    if cf_response.success {
                        if let Some(record) = cf_response.result {
                            let dns_record = CloudflareDnsRecord { 
                                id: record.id, 
                                name: record.name,
                                record_type: record.record_type,
                                content: record.content,
                                ttl: record.ttl,
                                proxied: record.proxied
                            };
                                
                            HttpResponse::Created().json(serde_json::json!({
                                "status": "success",
                                "message": "DNS record created successfully",
                                "record": dns_record
                            }))
                        } else {
                            HttpResponse::InternalServerError().json(serde_json::json!({
                                "status": "error",
                                "message": "Record was created but no data was returned"
                            }))
                        }
                    } else {
                        let error_msg = format!("Cloudflare API error: {}", 
                            serde_json::to_string(&cf_response.errors).unwrap_or_else(|_| 
                                "Unknown error".to_string()
                            )
                        );
                        
                        error!("{}", error_msg);
                        HttpResponse::BadGateway().json(serde_json::json!({
                            "status": "error",
                            "message": error_msg
                        }))
                    }
                },
                Err(e) => {
                    error!("Failed to parse Cloudflare API response: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "status": "error",
                        "message": format!("Failed to parse API response: {}", e)
                    }))
                }
            }
        },
        Err(e) => {
            error!("Failed to connect to Cloudflare API: {}", e);
            HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to connect to Cloudflare API: {}", e)
            }))
        }
    }
} 