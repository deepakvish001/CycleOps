//! Cooperative boundary for audit repository.

use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Record { pub id: Uuid, pub tenant_id: Uuid, pub version: i64 }

#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn get(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<Option<Record>>;
    async fn save(&self, tenant_id: Uuid, record: Record) -> anyhow::Result<Record>;
    async fn list(&self, tenant_id: Uuid, limit: u32, cursor: Option<String>) -> anyhow::Result<Vec<Record>>;
}
