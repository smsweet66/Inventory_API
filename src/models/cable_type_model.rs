use base64::{engine::general_purpose::STANDARD, Engine as _};
use diesel::{Queryable, Insertable, AsChangeset};
use serde::{Deserialize, Serialize, Serializer, Deserializer};

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
	#[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
	pub image: Vec<u8>
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset)]
pub struct CableType {
	pub id: i32,
	pub name: String,
	pub cable_gender: Gender,
	#[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
	pub image: Vec<u8>
}

fn as_base64<T: AsRef<[u8]>, S: Serializer>(val: &T, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&STANDARD.encode(val))
}

fn from_base64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    use serde::de;

    <&str>::deserialize(deserializer).and_then(|s| {
        STANDARD.decode(s)
            .map_err(|e| de::Error::custom(format!("invalid base64 string: {}, {}", s, e)))
    })
}