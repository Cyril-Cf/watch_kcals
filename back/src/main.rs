use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/test")]
async fn get_test() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_test))
        .bind("0.0.0.0:3001")?
        .run()
        .await
}