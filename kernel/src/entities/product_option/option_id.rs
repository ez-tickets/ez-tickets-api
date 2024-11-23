use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entities::ProductId;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct OptionId(Uuid);

impl OptionId {
    pub fn new(id: impl Into<Uuid>) -> OptionId {
        Self(id.into())
    }
}

impl AsRef<Uuid> for OptionId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<OptionId> for Uuid {
    fn from(value: OptionId) -> Self {
        value.0
    }
}

impl From<OptionId> for ProductId {
    fn from(value: OptionId) -> Self {
        ProductId::new(value.0)
    }
}

impl From<ProductId> for OptionId {
    fn from(value: ProductId) -> Self {
        OptionId::new(value)
    }
}

impl Default for OptionId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for OptionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OptionId({})", self.0)
    }
}