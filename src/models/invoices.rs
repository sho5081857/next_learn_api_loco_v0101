use sea_orm::entity::prelude::*;
use super::_entities::invoices::{ActiveModel, Entity};
pub type Invoices = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
