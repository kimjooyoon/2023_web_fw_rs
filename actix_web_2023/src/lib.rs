pub mod router;
mod handler;
mod user;


#[cfg(test)]
mod tests {
    use actix_http::body::BoxBody;
    use actix_web::{body::to_bytes, dev::Service, http, test, App, Error};
    use actix_web::dev::ServiceResponse;
    use crate::handler::{index, name};
    use crate::user::handler::get_user;

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

    #[actix_web::test]
    async fn test_get_user() -> Result<(), Error> {
        let url = String::from("/users/tester");
        let resp = test_request(url).await?;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?,
                   "{\"name\":\"name\",\"email\":\"tester\"}");

        Ok(())
    }

    async fn test_request(url: String)
                          -> Result<ServiceResponse<BoxBody>, Error> {
        let app = App::new()
            .service(index)
            .service(name)
            .service(get_user);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri(&url).to_request();
        app.call(req).await
    }
}
