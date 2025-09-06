use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use crate::module::product::v1::entity::model::product;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "transactions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub product_id: i32,
    pub hpp_amount: i32,
    pub selling_amount: i32,
    pub qty: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::module::product::v1::entity::model::product::Entity",
        from = "Column::ProductId",
        to = "crate::module::product::v1::entity::model::product::Column::Id"
    )]
    Product,
}

impl Related<product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}