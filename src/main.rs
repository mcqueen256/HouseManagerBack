use actix_web::{
    middleware, web, App, HttpResponse, HttpServer,
};
// use futures::StreamExt;
// use json::JsonValue;
use serde::{Deserialize, Serialize};

use std::fs;

const DATABASE: &'static str = "db.json";

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    users: Vec<User>,
    homes: Vec<Home>,
}

impl Database {
    fn new() -> Self {
        Database {
            users: vec![],
            homes: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: usize,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    member_to: Vec<usize>, // Home.id
}

#[derive(Debug, Serialize, Deserialize)]
struct Home {
    id: usize,
    owner_id: usize, // User.id
    members: Vec<usize>, // [User.id]
    features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Feature {

}


use std::io::Write;
async fn pop() -> HttpResponse {
    println!("sending database...");
    let contents= fs::read_to_string(DATABASE).unwrap_or_else(|e| {
        eprintln!("Error: {:?}", e);
        eprintln!("Creating (a) a new file...");
        let mut file = fs::File::create(DATABASE).expect("could not create new file db.json");
        let contents = serde_json::to_string(&Database::new()).expect("coule not create a json from Database::new()");
        file.write_all(contents.as_bytes()).expect("could not write to file");
        contents
    });
    let db: Database = serde_json::from_str(&contents).expect("could not build json from Database struct");
    HttpResponse::Ok().json(db) // <- send response
}

async fn push(db: web::Json<Database>) -> HttpResponse {
    println!("saving database...");
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(DATABASE)
        .expect("could not open file for writing");
    let contents = serde_json::to_string(&db.0).expect("failed to convert recieved object to json str");
    file.write_all(contents.as_bytes()).expect("could not write to file");
    HttpResponse::Ok().finish() // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/pop").route(web::post().to(pop)))
            .service(web::resource("/push").route(web::post().to(push)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}