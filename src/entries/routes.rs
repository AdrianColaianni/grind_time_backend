use crate::entries::{Entry, Entries};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[get("/example")]
async fn example() -> Result<HttpResponse, CustomError> {
    let entry = Entries::example();
    Ok(HttpResponse::Ok().json(entry))
}

#[get("/entries")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let entries = Entries::find_all()?;
    Ok(HttpResponse::Ok().json(entries))
}

#[derive(Serialize, Deserialize)]
struct EntryRequest {
    location: i32
}

#[get("/entries")]
async fn find(web::Query(info): web::Query<EntryRequest>) -> Result<HttpResponse, CustomError> {
    let entry = Entries::find(info.location)?;
    Ok(HttpResponse::Ok().json(entry))
}

#[post("/entries")]
async fn create(entry: web::Json<Entry>) -> Result<HttpResponse, CustomError> {
    let entry = Entries::create(entry.into_inner())?;
    Ok(HttpResponse::Ok().json(entry))
}

// #[put("/entries/{id}")]
// async fn update(
//     id: web::Path<i32>,
//     entry: web::Json<Entry>,
// ) -> Result<HttpResponse, CustomError> {
//     let entry = Entries::update(id.into_inner(), entry.into_inner())?;
//     Ok(HttpResponse::Ok().json(entry))
// }

#[delete("/entries/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_entry = Entries::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_entry })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(example);
    config.service(find_all);
    config.service(find);
    config.service(create);
    // config.service(update);
    config.service(delete);
}
