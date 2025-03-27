use cloudflare_ddns::{
    run_server, 
    ServiceFactory, 
    DdnsConfig,
    Settings,
    IpType
};
use log::{info, error};
use std::env;
use std::process;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

// 用於控制 DDNS 服務的全局變量
static DDNS_CONTROL: once_cell::sync::Lazy<Arc<Mutex<Option<mpsc::Sender<()>>>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

// 重啟 DDNS 服務的公共函數
pub fn restart_ddns_service() {
    if let Ok(mut ctrl) = DDNS_CONTROL.lock() {
        if let Some(sender) = ctrl.take() {
            // 發送停止信號給當前服務
            let _ = sender.send(());
        }
        
        // 啟動新的 DDNS 服務
        start_ddns_service();
    }
}

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
    
    // 建立服務工廠
    let service_factory = ServiceFactory::new();
    
    // 從環境變數載入配置
    let configs = match load_ddns_configs_from_env() {
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
        
        // 建立 DDNS 應用服務
        let ddns_service = service_factory.create_ddns_service(ddns_config);
        
        // 啟動自動更新任務
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

/// 從環境變數載入 DDNS 配置
fn load_ddns_configs_from_env() -> Result<Vec<DdnsConfig>, String> {
    let mut configs = Vec::new();
    
    // 載入 IPv4 配置
    if let (Ok(api_token), Ok(zone_id), Ok(record_id), Ok(record_name)) = (
        env::var("CLOUDFLARE_API_TOKEN"),
        env::var("CLOUDFLARE_ZONE_ID"),
        env::var("CLOUDFLARE_RECORD_ID"),
        env::var("CLOUDFLARE_RECORD_NAME")
    ) {
        let update_interval = env::var("DDNS_UPDATE_INTERVAL")
            .map(|s| s.parse::<u64>().unwrap_or(300))
            .unwrap_or(300);
        
        let ipv4_config = DdnsConfig {
            api_token,
            zone_id,
            record_id,
            record_name,
            update_interval,
            ip_type: IpType::IPv4,
        };
        
        configs.push(ipv4_config);
    }
    
    // 載入 IPv6 配置
    if let (Ok(api_token), Ok(zone_id), Ok(record_id), Ok(record_name)) = (
        env::var("CLOUDFLARE_API_TOKEN"),
        env::var("CLOUDFLARE_ZONE_ID"),
        env::var("CLOUDFLARE_RECORD_ID_V6"),
        env::var("CLOUDFLARE_RECORD_NAME_V6")
    ) {
        let update_interval = env::var("DDNS_UPDATE_INTERVAL")
            .map(|s| s.parse::<u64>().unwrap_or(300))
            .unwrap_or(300);
        
        let ipv6_config = DdnsConfig {
            api_token,
            zone_id,
            record_id,
            record_name,
            update_interval,
            ip_type: IpType::IPv6,
        };
        
        configs.push(ipv6_config);
    }
    
    Ok(configs)
}