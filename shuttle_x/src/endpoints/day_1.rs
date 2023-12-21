use actix_web::{web::{self, Path}, get, HttpResponse, App, HttpServer};
use serde::Deserialize;

#[get("/1/{num1}/{num2}")]
async fn calculate_xor_pow3(info: web::Path<(u32, u32)>) -> HttpResponse {
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

/*#[get("/1/{nums}")]
async fn multiple_xor_pow3(info: web::Path<String>) -> HttpResponse {
    let nums: Vec<u32> = info.into_inner().split('/').filter_map(|s| s.parse().ok()).collect();
    let result = nums.iter().fold(0, |acc, &num| acc ^ num).pow(3);
    HttpResponse::Ok().json(result)
}*/

#[get("/1/{input_1}/{inputs:.*}")]
async fn multiple_xor_pow3(path: Path<(i32,String)>) -> HttpResponse {
    let (input1,others) = path.into_inner();
    let remaining_inputs = others.split('/').map(|item| item.parse::<i32>().unwrap_or_default()).collect::<Vec<i32>>();
    let mut response_xor = input1;

    for item in remaining_inputs{
        response_xor ^= item;
    }
    let powed_input = response_xor.pow(3);
    HttpResponse::Ok().json(powed_input)
}
