#[derive(Debug, PartialEq, DbEnum)]
#[DieselType = "Status"]
pub enum Status {
    unclaimed,
    unfinished,
    unreviewed,
    completed,
}
