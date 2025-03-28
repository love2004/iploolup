// 基礎設施層配置模塊將在後續階段實現 

// 配置模塊的基礎設施實現

// 從文件系統加載和保存配置的功能
mod repository;
pub use repository::FileConfigRepository; 