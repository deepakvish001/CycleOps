//! settlement domain model.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settlement {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub reference: String,
    pub status: String,
}

impl Settlement {
    pub fn new(tenant_id: Uuid, reference: impl Into<String>) -> Result<Self, &'static str> {
        let reference = reference.into();
        if reference.trim().is_empty() { return Err("reference is required"); }
        Ok(Self { id: Uuid::new_v4(), tenant_id, reference, status: "active".into() })
    }

    pub fn belongs_to(&self, tenant_id: Uuid) -> bool { self.tenant_id == tenant_id }
}
