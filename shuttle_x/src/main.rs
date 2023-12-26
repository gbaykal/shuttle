pub mod endpoints;

use actix_web::{get, web::{ServiceConfig, self}};
use endpoints::{day_1, day0::hello_world, day_4, day_5, day_6, day_7, day_8, day_11,day_13};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool,) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    
    let state = web::Data::new(AppState { pool });
    
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(day_1::calculate_xor_pow3);
        cfg.service(day_1::multiple_xor_pow3);
        cfg.service(day_4::calculate_combined_strength);
        cfg.service(day_5::paginate_names);
        cfg.service(day_6::count_elf_endpoint);
        cfg.service(day_7::decode_cookie);
        cfg.service(day_8::pokemon_weight);
        cfg.service(day_11::png_endpoint);
        cfg.service(day_13::sql_endpoint);
        cfg.app_data(state);
    };

    Ok(config.into())
}