use cloudflare_ddns::config::Settings;
use cloudflare_ddns::config::DdnsConfigLoader;
use cloudflare_ddns::run_server;
use cloudflare_ddns::services::ddns::DdnsService;
use log::{info, error};
use std::env;
use std::process;

/// 啟動 DDNS 服務（作為獨立進程）
fn start_ddns_service() {
    // 使用獨立進程啟動 DDNS 服務
    let child = match process::Command::new(env::current_exe().unwrap())
        .env("RUST_LOG", env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .env("RUN_MODE", "ddns")
        .spawn() {
            Ok(child) => child,
            Err(e) => {
                error!("Failed to start DDNS service process: {}", e);
                return;
            }
        };
    
    info!("DDNS service started in a separate process (PID: {})", child.id());
}

/// 應用程式入口點
/// 
/// # 功能
/// 
/// - 載入環境變數
/// - 初始化日誌系統
/// - 載入應用程式設置
/// - 配置並啟動 DDNS 服務
/// - 啟動 Web 伺服器
/// 
/// # 環境變數
/// 
/// - `RUST_LOG`: 日誌級別（默認：info）
/// - `CLOUDFLARE_API_TOKEN`: Cloudflare API 令牌
/// - `CLOUDFLARE_ZONE_ID`: Cloudflare 區域 ID
/// - `CLOUDFLARE_RECORD_ID`: IPv4 DNS 記錄 ID
/// - `CLOUDFLARE_RECORD_NAME`: IPv4 DNS 記錄名稱
/// - `CLOUDFLARE_RECORD_ID_V6`: IPv6 DNS 記錄 ID（可選）
/// - `CLOUDFLARE_RECORD_NAME_V6`: IPv6 DNS 記錄名稱（可選）
/// - `DDNS_UPDATE_INTERVAL`: 更新間隔（秒，默認：300）
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 載入 .env 檔案
    dotenv::dotenv().ok();
    
    // 設置日誌
    if env::var("RUST_LOG").is_err() {
        unsafe {
            env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();
    
    // 檢查運行模式
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "web".to_string());
    
    if run_mode == "ddns" {
        // 在 DDNS 模式下運行
        return run_ddns_service().await;
    } else {
        // 在 Web 模式下運行
        // 先啟動 DDNS 服務作為獨立進程
        start_ddns_service();
        
        // 載入設置
        let settings = Settings::new().expect("Failed to load settings");
        
        // 運行 Web 伺服器
        info!("Starting Web server at {}:{}", settings.server.host, settings.server.port);
        run_server(&settings.server.host, settings.server.port).await?;
    }
    
    Ok(())
}

/// 運行 DDNS 服務
async fn run_ddns_service() -> std::io::Result<()> {
    info!("Starting DDNS service...");
    
    // 載入配置
    let configs = match DdnsConfigLoader::load_all_configs() {
        Ok(configs) => configs,
        Err(e) => {
            error!("Failed to load DDNS configuration: {}", e);
            return Ok(()); // 優雅退出
        }
    };

    if configs.is_empty() {
        error!("No available DDNS configurations, service exiting");
        return Ok(());
    }
    
    info!("Successfully loaded {} DDNS configurations", configs.len());
    
    let mut tasks = Vec::new();
    
    // 啟動所有配置的 DDNS 服務
    for ddns_config in configs {
        let ip_type = ddns_config.ip_type.clone();
        info!("Starting {} DDNS update service", ip_type);
        
        // 啟動 DDNS 自動更新任務
        let ddns_service = DdnsService::new(ddns_config);
        let handle = tokio::spawn(async move {
            ddns_service.start_auto_update().await;
        });
        tasks.push(handle);
    }
    
    // 等待 Ctrl+C 信號
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            info!("Received termination signal, shutting down DDNS service...");
        }
        Err(err) => {
            error!("Failed to listen for termination signal: {}", err);
        }
    }
    
    Ok(())
}