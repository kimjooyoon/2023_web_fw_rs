use actix_web::{get, HttpRequest, Responder, web};

#[get("/")]
pub async fn index(req: HttpRequest) -> impl Responder {
    println!("REQ: {req:?}");
    "hi teddy!"
}

#[get("/{name}")]
pub async fn name(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

