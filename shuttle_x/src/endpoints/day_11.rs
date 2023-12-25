use actix_web::{web, HttpResponse, Responder, get, App, HttpRequest};
use actix_files::NamedFile;



#[get("/11/assets/decoration.png")]
async fn png_endpoint (_req: HttpRequest) -> impl Responder {
    NamedFile::open_async("assets/decoration.png").await
}