use diesel::sql_types::Datetime;
use diesel_derive_enum::DbEnum;

#[derive(Queryable, Debug)]
pub struct Difficulty {
    pub id: i32,
    pub name: String,
    pub multiplier: f64,
    pub score_requirement: i32,
}

#[derive(DbEnum)]
pub enum Status {
    Unclaimed,
    Unfinished,
    Unreviewed,
    Completed,
}

#[derive(Insertable, Queryable, Identifiable, Debug, PartialEq)]
pub struct PlotsystemPlots {
    id: i32,
    city_project_id: i32,
    difficulty_id: i32,
    review_id: i32,
    owner_uuid: String,
    member_uuids: String,
    status: Status,
    mc_coordinates: String,
    score: i32,
    last_activity: Datetime,
    create_date: Datetime,
    create_player: String,
    pasted: bool,
}
