use actix_web::{web, HttpResponse, Responder, get, App, HttpRequest, error};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

use crate::AppState;



#[get("/13/sql")] 
    async fn sql_endpoint(state: web::Data<AppState>) -> HttpResponse {
        let response = sqlx::query!("SELECT 20231213 number")
        .fetch_one(&state.pool).await.unwrap().number.unwrap();
        HttpResponse::Ok().json(response)
    }
