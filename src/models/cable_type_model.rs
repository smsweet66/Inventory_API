use diesel::{Queryable, Insertable, AsChangeset};
use serde::{Deserialize, Serialize};
use crate::schema::cable_types;

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Gender"]
pub enum Gender {
	Male,
	Female
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = cable_types)]
pub struct NewCableType {
	pub name: String,
	pub cable_gender: Gender,
	pub image: Vec<u8>
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset)]
pub struct CableType {
	pub id: i32,
	pub name: String,
	pub cable_gender: Gender,
	pub image: Vec<u8>
}