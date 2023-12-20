pub mod endpoints;

use actix_web::{get, web::ServiceConfig};
use endpoints::{day_1, day0::hello_world};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(day_1::calculate_xor_pow3)
;
    };

    Ok(config.into())
}