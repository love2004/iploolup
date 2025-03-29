use cloudflare_ddns::application::events::{EventManager, EventType, EventData, EventListener};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;
use std::time::Duration;
use std::collections::HashMap;
use tokio::time::sleep;

// 測試用的事件監聽器
#[derive(Debug, Default)]
struct TestEventListener {
    received_events: Arc<Mutex<Vec<EventType>>>,
}

impl TestEventListener {
    fn new() -> Self {
        Self {
            received_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_received_events(&self) -> Vec<EventType> {
        let events = self.received_events.lock().unwrap();
        events.clone()
    }
}

#[async_trait]
impl EventListener for TestEventListener {
    async fn handle_event(&self, event: EventData) {
        let mut events = self.received_events.lock().unwrap();
        events.push(event.event_type);
    }
}

#[tokio::test]
async fn test_event_manager_register_listener() {
    // 創建事件管理器
    let event_manager = EventManager::new();
    
    // 創建並註冊事件監聽器
    let listener = Arc::new(TokioMutex::new(TestEventListener::new()));
    event_manager.register_listener(listener.clone()).await;
    
    // 發佈事件
    let event = EventData {
        event_type: EventType::RestartDdnsService,
        data: None,
    };
    event_manager.publish(event).await;
    
    // 等待短暫時間確保事件被處理
    sleep(Duration::from_millis(50)).await;
    
    // 檢查監聽器是否收到事件
    let listener_guard = listener.lock().await;
    let received_events = listener_guard.get_received_events();
    assert_eq!(received_events.len(), 1);
    assert_eq!(received_events[0], EventType::RestartDdnsService);
}

#[tokio::test]
async fn test_event_manager_subscribe() {
    // 創建事件管理器
    let event_manager = EventManager::new();
    
    // 訂閱特定事件類型
    let mut receiver = event_manager.subscribe(EventType::ForceUpdateDns).await.unwrap();
    
    // 發佈一個訂閱的事件和一個未訂閱的事件
    let event1 = EventData {
        event_type: EventType::ForceUpdateDns,
        data: Some("updated".to_string()),
    };
    event_manager.publish(event1).await;
    
    let event2 = EventData {
        event_type: EventType::RestartDdnsService,
        data: None,
    };
    event_manager.publish(event2).await;
    
    // 等待短暫時間確保事件被處理
    let received_event = tokio::time::timeout(Duration::from_millis(100), receiver.recv()).await;
    
    // 檢查是否接收到訂閱的事件
    assert!(received_event.is_ok());
    let event = received_event.unwrap().unwrap();
    assert_eq!(event.event_type, EventType::ForceUpdateDns);
    assert_eq!(event.data, Some("updated".to_string()));
}

#[tokio::test]
async fn test_event_manager_multiple_listeners() {
    // 創建事件管理器
    let event_manager = EventManager::new();
    
    // 創建兩個事件監聽器
    let listener1 = Arc::new(TokioMutex::new(TestEventListener::new()));
    let listener2 = Arc::new(TokioMutex::new(TestEventListener::new()));
    
    // 註冊監聽器
    event_manager.register_listener(listener1.clone()).await;
    event_manager.register_listener(listener2.clone()).await;
    
    // 發佈事件
    let event = EventData {
        event_type: EventType::ForceUpdateDns,
        data: None,
    };
    event_manager.publish(event).await;
    
    // 等待短暫時間確保事件被處理
    sleep(Duration::from_millis(50)).await;
    
    // 檢查兩個監聽器都接收到事件
    let listener_guard1 = listener1.lock().await;
    let received_events1 = listener_guard1.get_received_events();
    assert_eq!(received_events1.len(), 1);
    assert_eq!(received_events1[0], EventType::ForceUpdateDns);
    
    let listener_guard2 = listener2.lock().await;
    let received_events2 = listener_guard2.get_received_events();
    assert_eq!(received_events2.len(), 1);
    assert_eq!(received_events2[0], EventType::ForceUpdateDns);
}

#[tokio::test]
async fn test_event_data_with_payload() {
    // 創建事件管理器
    let event_manager = EventManager::new();
    
    // 創建一個帶有數據的事件
    let event = EventData {
        event_type: EventType::ForceUpdateDns,
        data: Some("192.168.1.1".to_string()),
    };
    
    // 訂閱事件
    let mut receiver = event_manager.subscribe(EventType::ForceUpdateDns).await.unwrap();
    
    // 發佈帶有數據的事件
    event_manager.publish(event).await;
    
    // 接收事件
    let received_event = tokio::time::timeout(Duration::from_millis(100), receiver.recv()).await;
    
    // 檢查接收到的數據
    assert!(received_event.is_ok());
    let event = received_event.unwrap().unwrap();
    assert_eq!(event.event_type, EventType::ForceUpdateDns);
    assert_eq!(event.data, Some("192.168.1.1".to_string()));
}

#[tokio::test]
async fn test_unregister_listener() {
    // 目前 EventManager 沒有實現 unregister_listener 方法，此測試暫時跳過
    // 等待 EventManager 實現後再啟用
}

#[tokio::test]
async fn test_event_manager_helper_methods() {
    // 創建事件管理器
    let event_manager = Arc::new(EventManager::new());
    
    // 創建監聽器並註冊
    let listener = Arc::new(TokioMutex::new(TestEventListener::new()));
    event_manager.register_listener(listener.clone()).await;
    
    // 測試幫助方法 - 重啟服務
    event_manager.restart_ddns_service().await;
    
    // 測試幫助方法 - 強制更新 DNS
    event_manager.force_update_dns(None).await;
    
    // 等待短暫時間確保事件被處理
    sleep(Duration::from_millis(50)).await;
    
    // 檢查監聽器接收到的事件
    let listener_guard = listener.lock().await;
    let received_events = listener_guard.get_received_events();
    assert_eq!(received_events.len(), 2);
    
    // 事件應該按順序接收
    assert_eq!(received_events[0], EventType::RestartDdnsService);
    assert_eq!(received_events[1], EventType::ForceUpdateDns);
} 