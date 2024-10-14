use sea_orm::entity::prelude::*;
use super::_entities::customers::{ActiveModel, Entity};
pub type Customers = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
