use crate::{db_get, entities::*, pool::Db};
use rocket::http::Status;
use sea_orm::{ActiveModelTrait, ActiveValue::*};
use sea_orm_rocket::Connection;

#[put("/plot/set_pasted/<plot_id>?<pasted>")]
pub async fn set_pasted(conn: Connection<'_, Db>, plot_id: i32, pasted: i8) -> Status {
    let db = conn.into_inner();

    let mut plot: plotsystem_plots::ActiveModel =
        db_get::plot::by_plot_id(db, plot_id).await.into();

    plot.pasted = Set(pasted);

    return match plot.update(db).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    };
}
