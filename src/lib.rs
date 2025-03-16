pub mod api;
pub mod services;
pub mod config;
pub mod error;

use actix_web::{App, HttpServer, middleware::Logger};
use std::io;
use log::info;

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