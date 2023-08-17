use actix_web::web;
use diesel::{prelude::*, r2d2::{Pool, ConnectionManager}};

use crate::models::cable_model::*;
use crate::schema::cables;
use crate::handlers::cable_type_handlers::CableError;

type DB = web::Data<Pool<ConnectionManager<PgConnection>>>;

pub async fn get_cables_handler(db: DB) -> Result<Vec<Cable>, CableError> {
    let mut conn = db.get().unwrap();
    let res = cables::table.load::<Cable>(&mut conn);

    match res {
        Ok(cables) => Ok(cables),
        Err(_) => Err(CableError::DatabaseError)
    }
}

pub async fn create_cable_handler(db: DB, cable: NewCable) -> Result<Cable, CableError> {
    let mut conn = db.get().unwrap();
    let res = diesel::insert_into(cables::table)
        .values(&cable)
        .get_result::<Cable>(&mut conn);

    match res {
        Ok(cable) => Ok(cable),
        Err(_) => Err(CableError::DatabaseError)
    }
}

pub async fn update_cable_handler(db: DB, cable: Cable) -> Result<Cable, CableError> {
    let mut conn = db.get().unwrap();
    let res = diesel::update(cables::table)
        .filter(cables::id.eq(cable.id))
        .set(&cable)
        .get_result::<Cable>(&mut conn);

    match res {
        Ok(cable) => Ok(cable),
        Err(diesel::result::Error::NotFound) => Err(CableError::NotFound),
        Err(_) => Err(CableError::DatabaseError)
    }
}

pub async fn delete_cable_handler(db: DB, id: i32) -> Result<(), CableError> {
    let mut conn = db.get().unwrap();
    let res = diesel::delete(cables::table)
        .filter(cables::id.eq(id))
        .execute(&mut conn);

    match res {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::NotFound) => Err(CableError::NotFound),
        Err(_) => Err(CableError::DatabaseError)
    }
}