// API 模塊將在後續階段實現 

mod health;
mod ip;
mod status;
mod update;
mod router;
mod config;

pub use router::configure_routes; 