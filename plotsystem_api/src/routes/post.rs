use crate::{db_get, entities::*, pool::Db};
use rocket::{data::N, http::Status, serde::json::Json};
use sea_orm::{ActiveValue::*, EntityTrait};
use sea_orm_rocket::Connection;

#[post("/add_plot", format = "json", data = "<plot_json>")]
pub async fn add_plot(
    conn: Connection<'_, Db>,
    plot_json: Json<crate::entities::plotsystem_plots::Model>,
) -> Status {
    let db = conn.into_inner();

    // print!("{:?}", plot_json.id);

    let plot = crate::entities::plotsystem_plots::ActiveModel {
        id: NotSet,
        city_project_id: Set(plot_json.city_project_id.to_owned()),
        difficulty_id: Set(plot_json.difficulty_id.to_owned()),
        review_id: Set(plot_json.review_id.to_owned()),
        owner_uuid: Set(plot_json.owner_uuid.to_owned()),
        member_uuids: Set(plot_json.member_uuids.to_owned()),
        status: Set(plot_json.status.to_owned()),
        mc_coordinates: Set(plot_json.mc_coordinates.to_owned()),
        score: Set(plot_json.score.to_owned()),
        last_activity: Set(plot_json.last_activity.to_owned()),
        create_date: Set(plot_json.create_date.to_owned()),
        create_player: Set(plot_json.create_player.to_owned()),
        pasted: Set(plot_json.pasted.to_owned()),
    };

    print!("plot: {:#?}", plot_json);

    return match plotsystem_plots::Entity::insert(plot).exec(db).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    };
}
