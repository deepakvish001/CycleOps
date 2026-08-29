#![forbid(unsafe_code)]

pub mod tenant {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct TenantId(Uuid);

    impl TenantId {
        pub fn new(value: Uuid) -> Self {
            Self(value)
        }

        pub fn value(self) -> Uuid {
            self.0
        }
    }
}
