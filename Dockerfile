FROM rust:1.67 as builder

WORKDIR /usr/src/app
COPY . .

# 建立應用程式
RUN cargo build --release

# 執行階段映像
FROM debian:buster-slim

# 安裝必要的SSL證書和時區數據
RUN apt-get update && apt-get install -y \
    ca-certificates \
    tzdata \
    && rm -rf /var/lib/apt/lists/*

# 創建配置目錄
RUN mkdir -p /app/config /app/static

# 從構建階段複製二進制文件
COPY --from=builder /usr/src/app/target/release/cloudflare-ddns /app/
COPY --from=builder /usr/src/app/static/ /app/static/
COPY --from=builder /usr/src/app/config/ /app/config/

WORKDIR /app

# 設置環境變量
ENV RUST_LOG=info
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080

# 暴露端口
EXPOSE 8080

# 運行應用程式
CMD ["./cloudflare-ddns"] 