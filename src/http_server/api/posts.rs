use actix_web::{get, post, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{core::posts};
use super::super::AppState;

#[derive(Deserialize)]
struct GetAllQuery {
    limit: Option<u32>,
    skip: Option<u32>,
}

#[get("/")]
async fn get_all(app_state: web::Data<AppState>, query: web::Query<GetAllQuery>) -> impl Responder {
    let limit = match query.limit {
        Some(limit) => if limit > 30 || limit < 0 {30} else {limit},
        None => 30
    };

    let skip = match query.skip {
        Some(skip) => skip,
        None => 0,
    };

    match posts::get_all(app_state.db.clone(), limit, skip).await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(error) => {
            dbg!(error);
            HttpResponse::InternalServerError().json(json!({"message": "error"}))
        }
    }
}

#[get("/{id}")]
async fn get_one(app_state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match posts::get_one(app_state.db.clone(), *id).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(error) => {
            dbg!(error);
            HttpResponse::InternalServerError().json(json!({"message": "error"}))
        }
    }
}

#[derive(Deserialize)]
struct CreateBody {
    title: String,
    content: String,
    description: String,
}

#[post("/")]
async fn create(app_state: web::Data<AppState>, body: web::Json<CreateBody>) -> impl Responder {
    match posts::create(app_state.db.clone(), body.title.clone(), body.description.clone(), body.content.clone()).await {
        Ok(id) => HttpResponse::Ok().json(json!({
            "id": id
        })),
        Err(error) => {
            dbg!(error);
            HttpResponse::InternalServerError().json(json!({"message": "error"}))
        }
    }
}