use serde::Serialize;
use actix_http::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;

#[derive(Serialize)]
pub struct Users {
    name: String,
    email:  String,
}

impl Responder for Users {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

impl Users {
    pub fn new(name: String, email: String) -> Users {
        Users {
            name,
            email,
        }
    }
}