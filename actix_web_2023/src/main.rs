use actix_web::{App, get, HttpRequest, HttpServer, middleware, Responder, web};

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    println!("REQ: {req:?}");
    "hi teddy!"
}

#[get("/{name}")]
async fn name(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "hi teddy!" }))
            .service(name)
            .service(index)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_http::body::BoxBody;
    use actix_web::{body::to_bytes, dev::Service, http, test, App, Error};
    use actix_web::dev::ServiceResponse;

    use super::*;

    #[actix_web::test]
    async fn test_index() -> Result<(), Error> {
        let url = String::from("/");
        let resp = test_request(url).await?;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"hi teddy!"##);

        Ok(())
    }

    #[actix_web::test]
    async fn test_name() -> Result<(), Error> {
        let url = String::from("/name_test");
        let resp = test_request(url).await?;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, "Hello name_test!");

        Ok(())
    }

    async fn test_request(url : String)
        -> Result<ServiceResponse<BoxBody>, Error> {
        let app = App::new().service(index).service(name);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri(&url).to_request();
        app.call(req).await
    }
}
