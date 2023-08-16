use actix_web::web;
use diesel::{prelude::*, r2d2::{Pool, ConnectionManager}};

use crate::models::cable_type_model::*;
use crate::schema::cable_types;

type DB = web::Data<Pool<ConnectionManager<PgConnection>>>;

pub enum CableTypeError {
	NotFound,
	AlreadyExists,
	DatabaseError
}

impl CableTypeError
{
	pub fn message(&self) -> String {
		match self {
			CableTypeError::NotFound => "Cable Type not found".to_owned(),
			CableTypeError::AlreadyExists => "Cable Type already exists".to_owned(),
			CableTypeError::DatabaseError => "Database error".to_owned()
		}
	}
}

pub async fn get_cable_types_handler(db: DB) -> Result<Vec<CableType>, CableTypeError> {
    let mut conn = db.get().unwrap();
    let res = cable_types::table.load::<CableType>(&mut conn);
    
    match res {
        Ok(cable_types) => Ok(cable_types),
        Err(_) => Err(CableTypeError::DatabaseError)
    }
}

pub async fn create_cable_type_handler(db: DB, cable_type: NewCableType) -> Result<CableType, CableTypeError> {
	let mut conn = db.get().unwrap();
	let res = diesel::insert_into(cable_types::table)
		.values(&cable_type)
		.get_result::<CableType>(&mut conn);

	match res {
		Ok(cable_type) => Ok(cable_type),
		Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => Err(CableTypeError::AlreadyExists),
		Err(_) => Err(CableTypeError::DatabaseError)
	}
}

pub async fn update_cable_type_handler(db: DB, cable_type: CableType) -> Result<CableType, CableTypeError> {
	let mut conn = db.get().unwrap();
	let res = diesel::update(cable_types::table)
		.filter(cable_types::id.eq(cable_type.id))
		.set(&cable_type)
		.get_result::<CableType>(&mut conn);

	match res {
		Ok(cable_type) => Ok(cable_type),
		Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => Err(CableTypeError::AlreadyExists),
		Err(diesel::result::Error::NotFound) => Err(CableTypeError::NotFound),
		Err(_) => Err(CableTypeError::DatabaseError)
	}
}

pub async fn delete_cable_type_handler(db: DB, cable_type_id: i32) -> Result<(), CableTypeError> {
	let mut conn = db.get().unwrap();
	let res = diesel::delete(cable_types::table)
		.filter(cable_types::id.eq(cable_type_id))
		.execute(&mut conn);

	match res {
		Ok(_) => Ok(()),
		Err(diesel::result::Error::NotFound) => Err(CableTypeError::NotFound),
		Err(_) => Err(CableTypeError::DatabaseError)
	}
}