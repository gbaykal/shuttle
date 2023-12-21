use actix_web::{web, HttpResponse, Responder, post, App};
use serde::{Deserialize, Serialize};

/*#[derive(Deserialize)]
struct Count {
    elf: usize,
}*/

#[derive(Serialize)]
struct ResponseString {
    #[serde(rename(serialize = "elf"))]
    elf: usize,
    #[serde(rename(serialize = "elf on a shelf"))]
    elf_on_a_shelf: usize,
    #[serde(rename(serialize = "shelf with no elf on it"))]
    shelf_without_elf_on: usize,
}

#[post("/6")]
async fn count_elf_endpoint(input: web::Bytes) -> HttpResponse {
    let input_str = std::str::from_utf8(&input).unwrap();
    let count = input_str.matches("elf").count();
    let count_elf_on_a_shelf: usize = input_str.matches("elf on a shelf").count();
    let shelf_count: usize = input_str.matches("shelf").count();
    let count_shelf_wo_elf: usize = shelf_count - count_elf_on_a_shelf;
    //let elf_count = Count { elf: count };
    let response = ResponseString{elf: count, elf_on_a_shelf: count_elf_on_a_shelf, shelf_without_elf_on: count_shelf_wo_elf};

    HttpResponse::Ok().json(response)
}