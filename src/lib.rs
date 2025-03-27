pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;

// 重新導出常用類型和結構
pub use application::ServiceFactory;
pub use domain::config::{DdnsConfig, IpType, Settings};
pub use domain::error::DomainError;
pub use application::error::ApplicationError;

// 重新導出服務啟動函數（暫時保留）
pub use actix_web::{web, App, HttpServer};
pub use actix_cors::Cors;

/// 啟動 Web 伺服器
/// 
/// # 參數
/// 
/// - `host`: 伺服器監聽的主機地址
/// - `port`: 伺服器監聽的端口
/// 
/// # 返回
/// 
/// - `io::Result<()>`: 成功時返回 `()`，失敗時返回錯誤
/// 
/// # 功能
/// 
/// - 配置並啟動 HTTP 伺服器
/// - 設置日誌中間件
/// - 配置 API 路由
pub async fn run_server(host: &str, port: u16) -> std::io::Result<()> {
    let address = format!("{}:{}", host, port);
    
    // 創建服務工廠
    let service_factory = web::Data::new(ServiceFactory::new());
    
    HttpServer::new(move || {
        // 啟用 CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        
        App::new()
            .wrap(cors)
            // 注冊服務工廠
            .app_data(service_factory.clone())
            // 配置路由
            .configure(interfaces::api::configure_routes)
            .configure(interfaces::web::configure_routes)
    })
    .bind(address)?
    .run()
    .await
}