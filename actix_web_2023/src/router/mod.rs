use actix_web::{App, HttpServer, middleware, web};
use crate::handler::{index, name};

pub async fn app_run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "hi teddy!" }))
            .service(name)
            .service(index)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}