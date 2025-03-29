use cloudflare_ddns::{ServiceFactory, web, App};
use actix_web::test;
use serde_json::{Value};
use std::sync::Arc;
use cloudflare_ddns::interfaces::api;

#[actix_web::test]
async fn test_health_endpoint() {
    // 創建測試應用
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Arc::new(ServiceFactory::new())))
            .configure(api::configure_routes)
    ).await;
    
    // 發送請求並獲取響應
    let req = test::TestRequest::get().uri("/api/health").to_request();
    let resp = test::call_service(&app, req).await;
    
    // 檢查響應狀態
    assert_eq!(resp.status(), 200);
    
    // 檢查響應體
    let body = test::read_body(resp).await;
    let response: Value = serde_json::from_slice(&body).unwrap();
    
    // 驗證響應內容
    assert_eq!(response["status"], "ok");
    assert!(response.get("version").is_some());
}

#[actix_web::test]
async fn test_status_endpoint() {
    // 創建測試應用
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Arc::new(ServiceFactory::new())))
            .configure(api::configure_routes)
    ).await;
    
    // 發送請求並獲取響應
    let req = test::TestRequest::get().uri("/api/status").to_request();
    let resp = test::call_service(&app, req).await;
    
    // 檢查響應狀態
    assert_eq!(resp.status(), 200);
    
    // 檢查響應體
    let body = test::read_body(resp).await;
    let response: Value = serde_json::from_slice(&body).unwrap();
    
    // 驗證響應內容
    assert_eq!(response["status"], "running");
    assert!(response.get("version").is_some());
}

#[actix_web::test]
async fn test_ipv4_endpoint() {
    // 創建測試應用
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Arc::new(ServiceFactory::new())))
            .configure(api::configure_routes)
    ).await;
    
    // 發送請求並獲取響應
    let req = test::TestRequest::get().uri("/api/ip/ipv4").to_request();
    let resp = test::call_service(&app, req).await;
    
    // 檢查響應狀態 (注意: 在測試環境中可能無法獲取真實IP)
    assert!(resp.status().is_success() || resp.status().is_server_error());
}

#[actix_web::test]
async fn test_ipv6_endpoint() {
    // 創建測試應用
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Arc::new(ServiceFactory::new())))
            .configure(api::configure_routes)
    ).await;
    
    // 發送請求並獲取響應
    let req = test::TestRequest::get().uri("/api/ip/ipv6").to_request();
    let resp = test::call_service(&app, req).await;
    
    // 檢查響應狀態 (注意: 在測試環境中可能無法獲取真實IP)
    assert!(resp.status().is_success() || resp.status().is_server_error());
}

#[actix_web::test]
async fn test_update_endpoint() {
    // 創建測試應用
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Arc::new(ServiceFactory::new())))
            .configure(api::configure_routes)
    ).await;
    
    // 發送請求並獲取響應
    let req = test::TestRequest::post().uri("/api/update").to_request();
    let resp = test::call_service(&app, req).await;
    
    // 檢查響應狀態 (在測試環境中可能無法執行實際更新)
    assert!(resp.status().is_success() || resp.status().is_server_error());
}

#[actix_web::test]
async fn test_restart_endpoint() {
    // 創建測試應用
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Arc::new(ServiceFactory::new())))
            .configure(api::configure_routes)
    ).await;
    
    // 發送請求並獲取響應
    let req = test::TestRequest::post().uri("/api/restart").to_request();
    let resp = test::call_service(&app, req).await;
    
    // 檢查響應狀態 (在測試環境中可能無法執行實際重啟)
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_configs_endpoint() {
    // 創建測試應用
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Arc::new(ServiceFactory::new())))
            .configure(api::configure_routes)
    ).await;
    
    // 發送請求並獲取響應
    let req = test::TestRequest::get().uri("/api/config").to_request();
    let resp = test::call_service(&app, req).await;
    
    // 檢查響應狀態
    assert_eq!(resp.status(), 200);
    
    // 檢查響應體
    let body = test::read_body(resp).await;
    let response: Value = serde_json::from_slice(&body).unwrap();
    
    // 驗證響應內容
    assert!(response.get("success").is_some());
    assert!(response.get("message").is_some());
    assert!(response.get("configs").is_some());
} 