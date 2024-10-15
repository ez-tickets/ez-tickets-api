mod description;
mod id;
mod name;
mod price;
mod stock;

pub use crate::entities::category::*;
pub use self::description::*;
pub use self::id::*;
pub use self::name::*;
pub use self::price::*;
pub use self::stock::*;

use destructure::{Destructure, Mutation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct Product {
    id: ProductId,
    name: ProductName,
    description: ProductDescription,
    stock: Stock,
    price: Price,
}

impl Product {
    pub fn create(
        id: ProductId,
        name: ProductName,
        description: ProductDescription,
        stock: Stock,
        price: Price,
    ) -> Self {
        Self {
            id,
            name,
            description,
            stock,
            price,
        }
    }
}

impl Product {
    pub fn id(&self) -> &ProductId {
        &self.id
    }
    
    pub fn name(&self) -> &ProductName {
        &self.name
    }

    pub fn description(&self) -> &ProductDescription {
        &self.description
    }

    pub fn stock(&self) -> &Stock {
        &self.stock
    }

    pub fn price(&self) -> &Price {
        &self.price
    }
}
