use iploolup::config::Settings;
use iploolup::run_server;
use log::info;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 設置日誌
    if env::var("RUST_LOG").is_err() {
        unsafe {
            env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();
    
    let settings = Settings::new().expect("Failed to load settings");
    
    info!("Starting server at {}:{}", settings.server.host, settings.server.port);
    
    run_server(&settings.server.host, settings.server.port).await
}