use diesel_derive_enum::DbEnum;

#[derive(DbEnum)]
pub enum EnumField {
    A,
    B,
    C,
}

#[derive(Insertable, Queryable, Identifiable)]
// #[diesel(table_name = table_with_enum)]
pub struct TableWithEnum {
    id: i32,
    enum_field: EnumField,
}
