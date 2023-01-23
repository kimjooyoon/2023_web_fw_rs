use crate::user::{dto};
use actix_web::{get, HttpRequest, Responder, web};

#[get("/users/{user}")]
pub async fn get_user(_req: HttpRequest, name: web::Path<String>) -> impl Responder {
    dto::Users::new("name".to_string(),name.to_string())
}


