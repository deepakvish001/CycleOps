//! Infrastructure adapter for nats jetstream.

use serde_json::Value;
use tracing::{info, instrument};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AdapterResult { pub tenant_id: Uuid, pub adapter: &'static str, pub payload: Value }

#[instrument(skip(payload), fields(tenant_id = %tenant_id))]
pub async fn execute(tenant_id: Uuid, payload: Value) -> Result<AdapterResult, &'static str> {
    if tenant_id.is_nil() { return Err("tenant scope is required"); }
    info!("nats-jetstream adapter executed");
    Ok(AdapterResult { tenant_id, adapter: "nats-jetstream", payload })
}
