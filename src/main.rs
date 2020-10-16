use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use futures::StreamExt;
use json::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    users: Vec<User>,
    homes: Vec<Home>,
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

/// This handler uses json extractor
async fn index(item: web::Json<Database>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
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
            // .service(web::resource("/extractor").route(web::post().to(index)))
            // .service(
            //     web::resource("/extractor2")
            //         .data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
            //         .route(web::post().to(extract_item)),
            // )
            // .service(web::resource("/manual").route(web::post().to(index_manual)))
            // .service(web::resource("/mjsonrust").route(web::post().to(index_mjsonrust)))
            .service(web::resource("/").route(web::post().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}