use actix_web::{web, HttpResponse, Responder, post, App};
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
struct NameList {
    names: Vec<String>,
}

#[derive(Deserialize)]
struct PaginationParams {
    offset: Option<usize>,
    limit: Option<usize>,
}


#[post("/5")]
async fn paginate_names(data: web::Json<NameList>, query: web::Query<PaginationParams>) -> HttpResponse {
    let offset = query.offset.unwrap_or_default() as usize;
    let limit = query.limit.unwrap_or_default() as usize;

    let sliced_names = data.names.iter().skip(offset).take(limit).cloned().collect::<Vec<_>>();

    HttpResponse::Ok().json(sliced_names)
}