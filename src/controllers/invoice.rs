#![allow(clippy::unused_async)]
use axum::{debug_handler, extract::Query};
use loco_rs::prelude::*;
use migration::Expr;

use sea_orm::{Condition, JoinType, QueryOrder, QuerySelect, RelationTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{
        _entities::{
            customers,
            invoices::{self, Entity, Model},
        },
        invoices::{InvoiceCreateParams, InvoiceUpdateParams},
    },
    views::invoice::{GetAllLatestInvoice, GetFilteredInvoice},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Queries {
    pub query: Option<String>,
}

async fn load_item(ctx: &AppContext, id: Uuid) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn get_all_latest(_auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let invoices = Entity::find()
        .select_only()
        .columns([invoices::Column::Id, invoices::Column::Amount])
        .columns([
            customers::Column::Name,
            customers::Column::Email,
            customers::Column::ImageUrl,
        ])
        .join(JoinType::InnerJoin, invoices::Relation::Customer.def())
        .order_by_desc(invoices::Column::Date)
        .into_model::<GetAllLatestInvoice>()
        .all(&ctx.db)
        .await?;

    format::json(invoices)
}

#[debug_handler]
pub async fn get_filtered(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(queries): Query<Queries>,
) -> Result<Response> {
    let query = queries.query.unwrap_or_default();
    let invoices = Entity::find()
        .select_only()
        .columns([
            invoices::Column::Id,
            invoices::Column::CustomerId,
            invoices::Column::Amount,
            invoices::Column::Date,
            invoices::Column::Status,
        ])
        .columns([
            customers::Column::Name,
            customers::Column::Email,
            customers::Column::ImageUrl,
        ])
        .join(JoinType::InnerJoin, invoices::Relation::Customer.def())
        .filter(
            Condition::any()
                .add(customers::Column::Name.contains(query.to_lowercase()))
                .add(customers::Column::Email.contains(query.to_lowercase()))
                .add(Expr::cust(format!(
                    "CAST(invoices.amount AS TEXT) LIKE '%{}%'",
                    query
                )))
                .add(Expr::cust(format!(
                    "CAST(invoices.date AS TEXT) LIKE '%{}%'",
                    query
                ))),
        )
        .order_by_desc(invoices::Column::Date)
        .into_model::<GetFilteredInvoice>()
        .all(&ctx.db)
        .await?;

    format::json(invoices)
}

#[debug_handler]
pub async fn get_count(_auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(
        Entity::find()
            .select_only()
            .column_as(invoices::Column::Id.count(), "count")
            .into_tuple::<i64>()
            .one(&ctx.db)
            .await?,
    )
}

#[debug_handler]
pub async fn get_status_count(_auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let pending = Entity::find()
        .select_only()
        .column_as(invoices::Column::Id.count(), "count")
        .filter(invoices::Column::Status.eq("pending"))
        .into_tuple::<i64>()
        .one(&ctx.db)
        .await?;
    let paid = Entity::find()
        .select_only()
        .column_as(invoices::Column::Id.count(), "count")
        .filter(invoices::Column::Status.eq("paid"))
        .into_tuple::<i64>()
        .one(&ctx.db)
        .await?;
    format::json(serde_json::json!({"pending": pending, "paid": paid}))
}

#[debug_handler]
pub async fn get_pages(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(queries): Query<Queries>,
) -> Result<Response> {
    let query = queries.query.unwrap_or_default();
    let count = Entity::find()
        .join(JoinType::InnerJoin, invoices::Relation::Customer.def())
        .filter(
            Condition::any()
                .add(customers::Column::Name.contains(query.to_lowercase()))
                .add(customers::Column::Email.contains(query.to_lowercase()))
                .add(Expr::cust(format!(
                    "CAST(invoices.amount AS TEXT) LIKE '%{}%'",
                    query
                )))
                .add(Expr::cust(format!(
                    "CAST(invoices.date AS TEXT) LIKE '%{}%'",
                    query
                ))),
        )
        .select_only()
        .column_as(Expr::cust("COUNT(*)"), "count")
        .into_tuple::<i64>()
        .one(&ctx.db)
        .await?;

    format::json(count)
}

#[debug_handler]
pub async fn get_by_id(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(invoice_id): Path<Uuid>,
) -> Result<Response> {
    let invoice = Entity::find()
        .filter(invoices::Column::Id.eq(invoice_id))
        .one(&ctx.db)
        .await?;
    format::json(invoice)
}

#[debug_handler]
pub async fn create(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<InvoiceCreateParams>,
) -> Result<Response> {
    let res = invoices::Model::create(&ctx.db, &params).await;
    let invoice = match res {
        Ok(invoice) => invoice,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not create invoice",);
            return format::json(());
        }
    };
    format::json(invoice)
}

#[debug_handler]
pub async fn update(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<Uuid>,
    Json(params): Json<InvoiceUpdateParams>,
) -> Result<Response> {
    let res = invoices::Model::update(&ctx.db, id, &params).await;
    let invoice = match res {
        Ok(invoice) => invoice,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not update invoice",);
            return format::json(());
        }
    };
    format::json(invoice)
}

#[debug_handler]
pub async fn remove(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<Uuid>,
) -> Result<()> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    Ok(())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("invoices")
        .add("/latest", get(get_all_latest))
        .add("/filtered", get(get_filtered))
        .add("/count", get(get_count))
        .add("/statusCount", get(get_status_count))
        .add("/pages", get(get_pages))
        .add("/:invoiceId", get(get_by_id))
        .add("", post(create))
        .add("/:invoiceId", patch(update))
        .add("/:invoiceId", delete(remove))
}
