//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "plotsystem_difficulties")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub multiplier: f64,
    pub score_requirment: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::plotsystem_plots::Entity")]
    PlotsystemPlots,
}

impl Related<super::plotsystem_plots::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlotsystemPlots.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
