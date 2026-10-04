 use actix_web::{App, HttpServer};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct SignupInput{
    pub username: String,
    pub password: String,
}

#[post("/signup")]
async fn signup(input: web::Json<SignupInput>) -> impl Responder {
    HttpResponse::Ok().body("Signup successful!")
}


 #[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(signup)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
