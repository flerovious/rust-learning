// simple web api for calculator using actix-web

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Operation {
    a: i32,
    b: i32,
}

// index returns welcome message
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the simple calculator web api! See README.md for usage.")
}

// add returns the sum of two numbers
#[get("/add/{a}/{b}")]
async fn add(info: web::Path<Operation>) -> impl Responder {
    let res = calculator::add(info.a, info.b);
    HttpResponse::Ok().body(res.to_string())
}

// sub returns the difference of two numbers
#[get("/sub/{a}/{b}")]
async fn sub(info: web::Path<Operation>) -> impl Responder {
    let res = calculator::sub(info.a, info.b);
    HttpResponse::Ok().body(res.to_string())
}

// mul returns the product of two numbers
#[get("/mul/{a}/{b}")]
async fn mul(info: web::Path<Operation>) -> impl Responder {
    let res = calculator::mul(info.a, info.b);
    HttpResponse::Ok().body(res.to_string())
}

// div returns the quotient of two numbers
#[get("/div/{a}/{b}")]
async fn div(info: web::Path<Operation>) -> impl Responder {
    let res = calculator::div(info.a, info.b);
    HttpResponse::Ok().body(res.to_string())
}

// main starts the web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(add)
            .service(sub)
            .service(mul)
            .service(div)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
