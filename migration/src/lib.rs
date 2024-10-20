#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20231103_114510_notes;

mod m20241014_074729_revenues;
mod m20241014_090352_customers;
mod m20241014_100526_invoices;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20231103_114510_notes::Migration),
            Box::new(m20241014_074729_revenues::Migration),
            Box::new(m20241014_090352_customers::Migration),
            Box::new(m20241014_100526_invoices::Migration),
        ]
    }
}