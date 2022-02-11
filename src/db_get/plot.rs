use sea_orm::{DatabaseConnection, PaginatorTrait, QueryFilter};

use crate::entities::{prelude::*, *};

use sea_orm::entity::*;

pub async fn by_plot_id(db: &DatabaseConnection, plot_id: i32) -> plotsystem_plots::Model {
    let plot = PlotsystemPlots::find_by_id(plot_id)
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .into();

    return plot;
}

pub async fn all(db: &DatabaseConnection) -> Vec<plotsystem_plots::Model> {
    return PlotsystemPlots::find().all(db).await.unwrap();
}

pub async fn filtered(
    db: &DatabaseConnection,
    status: Option<crate::entities::sea_orm_active_enums::Status>,
    pasted: Option<bool>,
    limit: Option<u32>,
) -> Vec<plotsystem_plots::Model> {
    let mut plots = PlotsystemPlots::find();

    match status {
        Some(status) => plots = plots.filter(plotsystem_plots::Column::Status.eq(status)),
        None => {}
    }

    match pasted {
        Some(pasted) => plots = plots.filter(plotsystem_plots::Column::Pasted.eq(pasted)),
        None => {}
    }

    let plots_filtered;

    match limit {
        Some(limit) => plots_filtered = plots.paginate(db, limit as usize).fetch().await,
        None => plots_filtered = plots.all(db).await,
    }

    return plots_filtered.unwrap();
}
