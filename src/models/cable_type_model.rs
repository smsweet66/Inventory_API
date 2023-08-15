use diesel::{Queryable, Insertable, Associations};
use serde::{Deserialize, Serialize};
use crate::schema::cable_types;

enum Gender {
	Male,
	Female
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = cable_types)]
pub struct cable_type {
	pub name: String,
	pub gender: Gender,
	pub picture: 
}