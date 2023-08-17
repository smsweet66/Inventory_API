use actix_web::{HttpResponse, Responder, web, get, post, put, delete};
use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection};
use crate::{handlers::cable_handlers::*, models::cable_model::*};

type DB = web::Data<Pool<ConnectionManager<PgConnection>>>;

#[get("")]
pub async fn get_cables(db: DB) -> impl Responder {
    let res = get_cables_handler(db).await;

    match res {
        Ok(cables) => HttpResponse::Ok().json(cables),
        Err(e) => HttpResponse::InternalServerError().body(e.message())
    }
}

#[post("")]
pub async fn create_cable(db: DB, cable: web::Json<NewCable>) -> impl Responder {
    let res = create_cable_handler(db, cable.into_inner()).await;

    match res {
        Ok(cable) => HttpResponse::Ok().json(cable),
        Err(e) => HttpResponse::InternalServerError().body(e.message())
    }
}

#[put("")]
pub async fn update_cable(db: DB, cable: web::Json<Cable>) -> impl Responder {
    let res = update_cable_handler(db, cable.into_inner()).await;

    match res {
        Ok(cable) => HttpResponse::Ok().json(cable),
        Err(e) => HttpResponse::InternalServerError().body(e.message())
    }
}

#[delete("/{id}")]
pub async fn delete_cable(db: DB, id: web::Path<i32>) -> impl Responder {
    let res = delete_cable_handler(db, id.into_inner()).await;

    match res {
        Ok(cable) => HttpResponse::Ok().json(cable),
        Err(e) => HttpResponse::InternalServerError().body(e.message())
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cables);
    cfg.service(create_cable);
    cfg.service(update_cable);
    cfg.service(delete_cable);
}