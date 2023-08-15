// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "gender"))]
    pub struct Gender;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Gender;

    cable_types (id) {
        id -> Int4,
        name -> Varchar,
        cable_gender -> Gender,
        image -> Nullable<Bytea>,
    }
}

diesel::table! {
    cables (id) {
        id -> Int4,
        end_a -> Int4,
        end_b -> Int4,
        cable_length -> Float4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cable_types,
    cables,
);
