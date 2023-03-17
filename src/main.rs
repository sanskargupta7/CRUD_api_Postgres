use actix_web::{web, App, HttpServer};
use sqlx::{ postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;
mod models;

mod services;
use services::*;

mod database;
use database::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("Error building a connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(index)
            .service(get_all_data)
            .service(get_one_data)
            .service(create_data)
            .service(update_entry)
            .service(delete_entry)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}