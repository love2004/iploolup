use actix_web::{web, get, Error};
use actix_files::NamedFile;
use std::path::PathBuf;

/// 首頁處理器
/// 
/// # 返回
/// 
/// - `impl Responder`: 返回首頁 HTML 文件
#[get("/")]
pub async fn index() -> Result<NamedFile, Error> {
    let path: PathBuf = PathBuf::from("static/index.html");
    Ok(NamedFile::open(path)?)
}

/// 處理靜態資源
/// 
/// # 參數
/// 
/// - `path`: 資源路徑
/// 
/// # 返回
/// 
/// - `impl Responder`: 返回靜態資源文件
#[get("/{filename:.*}")]
pub async fn static_files(path: web::Path<String>) -> Result<NamedFile, Error> {
    let filename = path.into_inner();
    let path = PathBuf::from(format!("static/{}", filename));
    Ok(NamedFile::open(path)?)
}

/// 配置 Web UI 路由
/// 
/// # 參數
/// 
/// - `cfg`: 服務配置
/// 
/// # 功能
/// 
/// - 註冊 Web UI 路由
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ui")
            .service(index)
            .service(static_files)
    );
} 