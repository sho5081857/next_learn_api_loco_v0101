use super::_entities::invoices::{self, ActiveModel, Entity, Model};
use loco_rs::model::ModelResult;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue, IntoActiveModel};
use serde::{Deserialize, Serialize};
pub type Invoices = Entity;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InvoiceCreateParams {
    pub customer_id: Uuid,
    pub amount: i32,
    pub status: String,
    pub date: Date,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InvoiceUpdateParams {
    pub customer_id: Option<Uuid>,
    pub amount: Option<i32>,
    pub status: Option<String>,
    pub date: Option<Date>,
}

impl InvoiceUpdateParams {
    fn update(&self, item: &mut ActiveModel) {
        if let Some(customer_id) = self.customer_id {
            item.customer_id = Set(customer_id);
        }
        if let Some(amount) = self.amount {
            item.amount = Set(amount);
        }
        if let Some(status) = &self.status {
            item.status = Set(status.clone());
        }
        if let Some(date) = self.date {
            item.date = Set(date);
        }
    }
}


impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

impl super::_entities::invoices::Model {
    async fn load_item(db: &DatabaseConnection, id: Uuid) -> Result<Model> {
        let item = Entity::find_by_id(id).one(db).await?;
        item.ok_or_else(|| Error::NotFound)
    }
    pub async fn create(
        db: &DatabaseConnection,
        params: &InvoiceCreateParams,
    ) -> ModelResult<Self> {
        let invoice = invoices::ActiveModel {
            customer_id: ActiveValue::set(params.customer_id),
            amount: ActiveValue::set(params.amount),
            status: ActiveValue::set(params.status.clone()),
            date: ActiveValue::set(params.date),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(invoice)
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        params: &InvoiceUpdateParams,
    ) -> ModelResult<Self> {
        let item = Self::load_item(db, id).await;
        let item = match item {
            Ok(item) => item,
            Err(err) => {
                tracing::info!(message = err.to_string(), "could not load item",);
                return Err(ModelError::EntityNotFound);
            }
        };
        let mut item = item.into_active_model();
        params.update(&mut item);
        let invoice = item.update(db).await?;

        Ok(invoice)
    }
}
