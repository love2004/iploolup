// 測試模塊
mod config_loader_test;
mod dns_service_test;
mod http_client_test;
mod api_endpoints_test;
mod event_system_test;
mod ddns_service_test;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
} 