pub mod endpoints;

use actix_web::{get, web::ServiceConfig};
use endpoints::{day_1, day0::hello_world, day_4, day_5, day_6};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(day_1::calculate_xor_pow3);
        cfg.service(day_1::multiple_xor_pow3);
        cfg.service(day_4::calculate_combined_strength);
        cfg.service(day_5::paginate_names);
        cfg.service(day_6::count_elf_endpoint);
    };

    Ok(config.into())
}