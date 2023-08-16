use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use crate::schema::cables;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Cable {
    pub id: i32,
    pub end_a: i32,
    pub end_b: i32,
    pub cable_length: f32,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = cables)]
pub struct NewCable {
    pub end_a: i32,
    pub end_b: i32,
    pub cable_length: f32,
}