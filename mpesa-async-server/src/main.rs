use mpesa_async::c2b::{ValidateReply, ValidatorServerRequest};
use actix_web::{web, HttpServer, App, post, Responder, HttpResponse};
use dotenv::dotenv;
use std::env;

#[post("cb2_validate")]
async fn cb2_validate(valid: web::Json<ValidatorServerRequest>)-> impl Responder{
    println!("/c2b_validate : {:#?}",valid.into_inner());
    //we accept all input
    web::Json(ValidateReply::success())
}

#[post("/c2b_confirm")]
async fn c2b_confirm(body: String)-> impl Responder{
    println!("/c2b_confirm : {}",body);
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    let port: u16 = match env::var("PORT"){
        Ok(x)=> x.parse::<u16>().unwrap(),
        Err(_) => 7777
    };

    HttpServer::new(||{
        App::new().service(c2b_confirm).service(cb2_validate)
    }).bind(("0.0.0.0", port))?
    .run().await
}
