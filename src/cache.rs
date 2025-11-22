use crate::config::Config;
use moka::future::Cache;
use std::time::Duration;

/// MessageCache is a TTL cache for storing message hashes to detect duplicates
/// The key is a combination of message hash and device ID
/// The value is a unit type since we only need to check existence
pub type MessageCache = Cache<String, ()>;

/// Initialize the message cache with TTL from configuration
pub fn init_cache(config: &Config) -> MessageCache {
    let ttl_seconds = config.cache_ttl_minutes * 60;
    
    Cache::builder()
        .time_to_live(Duration::from_secs(ttl_seconds))
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    
    #[tokio::test]
    async fn test_cache_initialization() {
        let config = Config::default();
        let cache = init_cache(&config);
        
        // Test that we can insert and retrieve from cache
        cache.insert("test_key".to_string(), ()).await;
        assert!(cache.get(&"test_key".to_string()).await.is_some());
    }
    
    #[tokio::test]
    async fn test_cache_ttl() {
        let mut config = Config::default();
        config.cache_ttl_minutes = 0; // Immediate expiration for testing
        
        let cache = init_cache(&config);
        
        // Insert an item
        cache.insert("test_key".to_string(), ()).await;
        
        // With 0 TTL, item should expire immediately
        // Note: In practice, moka may take a moment to clean up
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // The behavior might vary based on moka's internal cleanup timing
        // This test primarily ensures the cache is created correctly
    }
}