use actix_web::{web, guard, App, HttpServer, HttpResponse, Responder, post};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Deserialize)]
pub struct Reindeer {
    name: String,
    strength: i32,
}

/*#[derive(Deserialize)]
pub struct ReindeerContest {
    name: String,
    strength: i32,
    speed: f64,
    height:i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    pub candies_eaten: i32
}
#[derive(Deserialize)]
struct ContestResult {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}*/

#[post("/4/strength")]
async fn calculate_combined_strength(data: web::Json<Vec<Reindeer>>) -> HttpResponse {
    let combined_strength: i32 = data.iter().map(|r| r.strength).sum();
    HttpResponse::Ok().json(combined_strength)
}

/*async fn contest_summary(data: web::Json<Vec<ReindeerContest>>) -> impl Responder {
    let mut sorted_by_speed = data.clone();
    sorted_by_speed.sort_by(|a, b| b.speed.partial_cmp(&a.speed).unwrap_or(Ordering::Equal));

    let fastest_reindeer = sorted_by_speed.get(0);

    let mut sorted_by_height = data;
    sorted_by_height.sort_by(|a, b| b.height.cmp(&a.height));

    let tallest_reindeer = sorted_by_height.get(0);

    let mut sorted_by_snow_magic = data;
    sorted_by_snow_magic.sort_by(|a, b| b.snow_magic_power.cmp(&a.snow_magic_power));

    let magician_reindeer = sorted_by_snow_magic.get(0);

    let mut sorted_by_consumer = data;
    sorted_by_consumer.sort_by(|a, b| b.candies_eaten.cmp(&a.candies_eaten));

    let consumer_reindeer = sorted_by_consumer.get(0);

    let result = ContestResult {
        fastest: format!("Speeding past the finish line with a speed of {} is {}", fastest_reindeer.speed, fastest_reindeer.name),
        tallest: format!("{} is standing tall with his {} cm wide antlers", tallest_reindeer.name, tallest_reindeer.antler_width),
        magician: format!("{} could blast you away with a snow magic power of {}", magician_reindeer.name, magician_reindeer.snow_magic_power),
        consumer: format!("{} ate lots of candies, but also some {}", consumer_reindeer.name, consumer_reindeer.favorite_food),
    };

    HttpResponse::Ok().json(result)
}
*/