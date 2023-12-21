use actix_web::{web::{self, Path}, get, HttpResponse, App, HttpServer};
use serde::Deserialize;

#[get("/1/{num1}/{num2}")]
async fn calculate_xor_pow3(info: web::Path<(i32, i32)>) -> HttpResponse {
    let (num1, num2) = info.into_inner();
    let result = (num1 ^ num2).pow(3);
    HttpResponse::Ok().json(result) 
} 
/*
#[derive(Deserialize)]
struct Info {
    path: String,
}*/

/*#[get("1/{input}")]
async fn multiple_xor_pow3(info: String) -> HttpResponse {
    let input = info;
    let line = input.split('/').map(|item| item.parse::<i32>().unwrap_or_default()).collect::<Vec<i32>>();
    let mut num1 = line[0];

    for num in &line[1..] {
        num1 = num1 ^ num;
    }

    let result = num1.pow(3);


    HttpResponse::Ok().json(result)
}*/

#[get("/1/{others:.*}")]
async fn multiple_xor_pow3(info: web::Path<String>) -> HttpResponse {
    //let first_num = info.clone().0;
    let nums: Vec<i32> = info.into_inner().split('/').filter_map(|s| s.parse().ok()).collect();
    println!("{:?}",nums);
    let result = nums.iter().fold(0, |acc, &num| acc ^ num).pow(3);
    HttpResponse::Ok().json(result)
}


