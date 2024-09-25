use actix_web::{get, middleware::Logger, web::scope, App, HttpResponse, HttpServer, Responder};
use utoipa_scalar::{Scalar, Servable};
pub mod hello {
    use actix_web::{get, HttpResponse, Responder};

    #[utoipa::path(
        get,
        path = "/hey",
        responses(
            (status = 200, description = "Pet found successfully", body = Pet),
            (status = NOT_FOUND, description = "Pet was not found")
        ),
        params(
            ("id" = u64, Path, description = "Pet database id to get Pet for"),
        )
    )]
    #[get("/hey")]
    async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
    }
}
use utoipa::OpenApi;

mod openapi {
    use crate::ApiDoc;
    use actix_web::{get, HttpResponse, Responder};
    use utoipa::OpenApi;
    #[get("")]
    async fn get_openapi() -> impl Responder {
        let spec = ApiDoc::openapi().to_json().unwrap();

        HttpResponse::Ok().json(spec)
    }
}
#[derive(OpenApi)]
#[openapi(paths(hello::manual_hello))]
struct ApiDoc;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(scope("/api").service(hello::manual_hello))
            .service(scope("/openapi").service(openapi::get_openapi))
            .service(Scalar::with_url("/scalar", ApiDoc::openapi()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
