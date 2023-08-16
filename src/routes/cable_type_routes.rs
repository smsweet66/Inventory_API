use actix_web::{HttpResponse, Responder, web, get, post, put, delete};
use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection};
use crate::{handlers::cable_type_handlers::*, models::cable_type_model::{NewCableType, CableType}};

type DB = web::Data<Pool<ConnectionManager<PgConnection>>>;

#[get("")]
pub async fn get_cable_types(db: DB) -> impl Responder {
    let res = get_cable_types_handler(db).await;

    match res {
        Ok(cable_types) => HttpResponse::Ok().json(cable_types),
        Err(e) => HttpResponse::InternalServerError().body(e.message())
    }
}

#[post("")]
pub async fn create_cable_type(db: DB, cable_type: web::Json<NewCableType>) -> impl Responder {
    let res = create_cable_type_handler(db, cable_type.into_inner()).await;

    match res {
        Ok(cable_type) => HttpResponse::Ok().json(cable_type),
        Err(e) => HttpResponse::InternalServerError().body(e.message())
    }
}

#[put("")]
pub async fn update_cable_type(db: DB, cable_type: web::Json<CableType>) -> impl Responder {
    let res = update_cable_type_handler(db, cable_type.into_inner()).await;

    match res {
        Ok(cable_type) => HttpResponse::Ok().json(cable_type),
        Err(e) => HttpResponse::InternalServerError().body(e.message())
    }
}

#[delete("/{id}")]
pub async fn delete_cable_type(db: DB, id: web::Path<i32>) -> impl Responder {
    let res = delete_cable_type_handler(db, id.into_inner()).await;

    match res {
        Ok(_) => HttpResponse::Ok().body("Cable type deleted"),
        Err(e) => HttpResponse::InternalServerError().body(e.message())
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cable_types);
    cfg.service(create_cable_type);
    cfg.service(update_cable_type);
    cfg.service(delete_cable_type);
}