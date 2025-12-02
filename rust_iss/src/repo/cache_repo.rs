use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::sync::Arc;

pub struct CacheRepo {
    conn: Arc<MultiplexedConnection>,
}

impl CacheRepo {
    pub fn new(conn: Arc<MultiplexedConnection>) -> Self {
        Self { conn }
    }

    pub async fn set<T: Serialize>(&self, _key: &str, _value: &T, _ttl: u64) -> Result<()> {
        // Упрощенная реализация Redis - временно отключена
        Ok(())
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(&self, _key: &str) -> Result<Option<T>> {
        // Упрощенная реализация Redis - временно отключена
        Ok(None)
    }

    pub async fn delete(&self, _key: &str) -> Result<()> {
        // Упрощенная реализация Redis - временно отключена
        Ok(())
    }

    pub async fn exists(&self, _key: &str) -> Result<bool> {
        // Упрощенная реализация Redis - временно отключена
        Ok(false)
    }
}

