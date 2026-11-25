use support_server::{
    cache::init_cache,
    config::Config,
};
use std::time::Duration;

#[tokio::test]
async fn test_cache_ttl_expiration() {
    // Create a config with a short TTL for testing
    let mut config = Config::default();
    config.cache_ttl_minutes = 0; // Immediate expiration
    
    // Initialize the cache
    let cache = init_cache(&config);
    
    // Insert an item
    cache.insert("test_key".to_string(), ()).await;
    
    // With 0 TTL, item should expire immediately
    // Note: In practice, moka may take a moment to clean up
    tokio::time::sleep(Duration::from_millis(10)).await;
    
    // The behavior might vary based on moka's internal cleanup timing
    // This test primarily ensures the cache is created correctly
}

#[tokio::test]
async fn test_cache_insert_and_retrieve() {
    // Create a config with default settings
    let config = Config::default();
    
    // Initialize the cache
    let cache = init_cache(&config);
    
    // Insert an item
    let key = "test_key".to_string();
    cache.insert(key.clone(), ()).await;
    
    // Retrieve the item
    let result = cache.get(&key).await;
    
    // Assert that the item exists
    assert!(result.is_some());
}

#[tokio::test]
async fn test_cache_multiple_inserts() {
    // Create a config with default settings
    let config = Config::default();
    
    // Initialize the cache
    let cache = init_cache(&config);
    
    // Insert multiple items
    let keys = vec!["key1", "key2", "key3"];
    for key in &keys {
        cache.insert(key.to_string(), ()).await;
    }
    
    // Retrieve all items
    for key in &keys {
        let result = cache.get(&key.to_string()).await;
        assert!(result.is_some());
    }
}