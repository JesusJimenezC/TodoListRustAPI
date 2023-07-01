use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use actix_web::web::Data;
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;

mod db;
mod handlers;

use db::db_conn;
use handlers::{lists_handlers, tasks_handlers};
use crate::db::db_conn::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool with a maximum of 10 connections
    let pool = establish_connection(&database_url);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .route("/", web::get().to(greet))
            .configure(lists_handlers::config)
            .configure(tasks_handlers::config)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn greet() -> impl actix_web::Responder {
    format!("Hello, Diesel + Actix + SQLite!")
}
