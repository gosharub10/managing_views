use actix_web::HttpResponse;
use super::content_loader;

pub async fn items() -> HttpResponse{
    let html_data = content_loader::read_file(String::from("./src/templates/main.html"));

    HttpResponse::Ok()
    .content_type("text/html; charset=utf8")
    .body(html_data)
}