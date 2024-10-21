use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use chrono::naive::NaiveDate as Date;
use uuid::Uuid;

#[derive(FromQueryResult, Debug, Deserialize, Serialize)]
pub struct GetAllLatestInvoice {
    pub id: Uuid,
    pub name: String,
    pub image_url: String,
    pub email: String,
    pub amount: i32,
}


#[derive(FromQueryResult, Debug, Deserialize, Serialize)]
pub struct GetFilteredInvoice {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub name: String,
    pub email: String,
    pub image_url: String,
    pub amount: i32,
    pub date: Date,
    pub status: String,
}
