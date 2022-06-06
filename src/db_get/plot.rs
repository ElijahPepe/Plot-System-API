use sea_orm::{DatabaseConnection, DbErr, PaginatorTrait, QueryFilter};

use crate::entities::{prelude::*, *};

use sea_orm::entity::*;

pub async fn by_plot_id(
    db: &DatabaseConnection,
    plot_id: i32,
) -> Result<plotsystem_plots::Model, DbErr> {
    match PlotsystemPlots::find_by_id(plot_id).one(db).await? {
        Some(plot) => Ok(plot),
        None => Err(DbErr::RecordNotFound(format!(
            "Plot with id {} does not exists",
            plot_id
        ))),
    }
}

pub async fn filtered(
    db: &DatabaseConnection,
    status: Option<sea_orm_active_enums::Status>,
    pasted: Option<bool>,
    limit: Option<u32>,
) -> Result<Vec<plotsystem_plots::Model>, DbErr> {
    let mut plots = PlotsystemPlots::find();

    match status {
        Some(status) => plots = plots.filter(plotsystem_plots::Column::Status.eq(status)),
        None => {}
    }

    match pasted {
        Some(pasted) => plots = plots.filter(plotsystem_plots::Column::Pasted.eq(pasted)),
        None => {}
    }

    match limit {
        Some(limit) => plots.paginate(db, limit as usize).fetch().await,
        None => plots.paginate(db, 20 as usize).fetch().await,
    }
}
