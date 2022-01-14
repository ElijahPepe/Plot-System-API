#[derive(Queryable)]
pub struct CityProject {
    pub id: i64,
    pub country_id: i64,
    pub name: String,
    pub description: String,
    pub visible: bool,
}
