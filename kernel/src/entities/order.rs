mod id;
mod item;
mod options;
mod product;
mod quantity;

pub use self::id::*;
pub use self::item::*;
pub use self::options::*;
pub use self::product::*;
pub use self::quantity::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderForm {
    id: OrderId,
    items: Vec<OrderItem>,
}

impl OrderForm {
    pub fn new(id: OrderId, items: Vec<OrderItem>) -> Self {
        Self { id, items }
    }
}

impl OrderForm {
    pub fn id(&self) -> &OrderId {
        &self.id
    }

    pub fn items(&self) -> &Vec<OrderItem> {
        &self.items
    }
}
