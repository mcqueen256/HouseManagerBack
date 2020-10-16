use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use futures::StreamExt;
use json::JsonValue;
use serde::{Deserialize, Serialize};

struct Database {
    users: Vec<User>,
    homes: Vec<Home>,
}

struct User {
    id: usize,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    member_to: Vec<usize>, // Home.id
}

struct Home {
    id: usize,
    owner_id: usize, // User.id
    members: Vec<usize>, // [User.id]
    features: Vec<Feature>,
}

struct Feature {

}



fn main() {

}