#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, extract::Query};
use loco_rs::prelude::*;
use migration::Expr;
use sea_orm::{Condition, JoinType, QueryOrder, QuerySelect, RelationTrait};
use serde::{Deserialize, Serialize};

use crate::{
    models::_entities::{
        customers::{self, Entity},
        invoices,
    },
    views::customer::GetFilteredCustomer,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Queries {
    pub query: Option<String>,
}

#[debug_handler]
pub async fn get_all(_auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn get_filtered(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(queries): Query<Queries>,
) -> Result<Response> {
    let query = queries.query.unwrap_or_default();

    let customers = Entity::find()
        .select_only()
        .columns([
            customers::Column::Id,
            customers::Column::Name,
            customers::Column::Email,
            customers::Column::ImageUrl,
        ])
        .column_as(invoices::Column::Id.count(), "total_invoices")
        .column_as(
            Expr::cust(
                "SUM(CASE WHEN invoices.status = 'pending' THEN invoices.amount ELSE 0 END)",
            ),
            "total_pending",
        )
        .column_as(
            Expr::cust("SUM(CASE WHEN invoices.status = 'paid' THEN invoices.amount ELSE 0 END)"),
            "total_paid",
        )
        .join(JoinType::LeftJoin, customers::Relation::Invoices.def())
        .filter(
            Condition::any()
                .add(customers::Column::Name.contains(query.to_lowercase()))
                .add(customers::Column::Email.contains(query.to_lowercase())),
        )
        .group_by(customers::Column::Id)
        .group_by(customers::Column::Name)
        .group_by(customers::Column::Email)
        .group_by(customers::Column::ImageUrl)
        .order_by_asc(customers::Column::Name)
        .into_model::<GetFilteredCustomer>()
        .all(&ctx.db)
        .await?;

    format::json(customers)
}

#[debug_handler]
pub async fn get_count(_auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(
        Entity::find()
            .select_only()
            .column_as(customers::Column::Id.count(), "count")
            .into_tuple::<i64>()
            .one(&ctx.db)
            .await?,
    )
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("customers")
        .add("", get(get_all))
        .add("/filtered", get(get_filtered))
        .add("/count", get(get_count))
}
