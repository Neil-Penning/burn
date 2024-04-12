use actix_web::{get, web, App, HttpServer, Result};
use chrono::prelude::*;

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    let local: DateTime<Local> = Local::now();
    let output: String = format!("{}: Welcome {}, user_id {}!", local, friend, user_id);
    println!("{}", output);
    Ok(output)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
