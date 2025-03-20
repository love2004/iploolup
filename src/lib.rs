pub mod api;
pub mod services;
pub mod config;
pub mod error;

use actix_web::{App, HttpServer, middleware::Logger};
use std::io;
use log::info;

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
pub async fn run_server(host: &str, port: u16) -> io::Result<()> {
    info!("Configuring server...");
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(api::configure_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}