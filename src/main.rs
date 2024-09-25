use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};
use apistos::{
    api_operation,
    app::OpenApiWrapper,
    info::Info,
    server::Server,
    spec::Spec,
    web::{get, resource, scope},
};
// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

#[api_operation(summary = "A manual hello world")]
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let spec = Spec {
            info: Info {
                title: "An API".to_string(),
                version: "1.0.0".to_string(),
                ..Default::default()
            },
            servers: vec![Server {
                url: "/api/v3".to_string(),

                ..Default::default()
            }
            ],
            
            ..Default::default()
        };
        App::new()
            .document(spec)
            .wrap(Logger::default())
            .service(scope("/api").service(resource("").route(get().to(manual_hello))))
            .build("/openapi.json")
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
