use actix_web::get;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}