use sea_orm::entity::prelude::*;
use super::_entities::revenues::{ActiveModel, Entity};
pub type Revenues = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
