use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use std::time::SystemTime;
use actix_web::http::header::Date;
use std::sync::Mutex;
use chrono::prelude::*;
use chrono::offset::LocalResult;
use crate::AppState;
use crate::models::todomodels::{ Todo, CreateEntryData, UpdateEntryData };

#[get("/")]
async fn index() -> String {
    "normal".to_string()
}

#[get("/todolist/data")]
async fn get_all_data(d: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as!(Todo, "SELECT id, username, description, time, date FROM todolist")
        .fetch_all(&d.db)
        .await
    {
        Ok(todolist) => HttpResponse::Ok().json(todolist),
        Err(_) => HttpResponse::NotFound().json("No todo found"),
    }
}

#[get("/todolist/data/{id}")]
async fn get_one_data(d: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    match sqlx::query_as!(
        Todo,
        "SELECT id, username, description, time, date FROM todolist WHERE id = $1",
        id
    )
        .fetch_one(&d.db)
        .await
    {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::NotFound().json("No todo found"),
    }
}

#[post("/todolist/data")]
async fn create_data(d: web::Data<AppState>, body: web::Json<CreateEntryData>) -> impl Responder {

    match sqlx::query_as!(
        Todo,
        "INSERT INTO todolist (username, description, time, date) VALUES ($1, $2, $3, $4) RETURNING id, username, description, time, date",
        body.username.to_string(),
        body.description.to_string(),
        Utc::now().timestamp().to_string(),
        Utc::now().date_naive().to_string()
    )
        .fetch_one(&d.db)
        .await
    {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().json("failed to create"),
    }

}

#[put("/todolist/data/{id}")]
async fn update_entry(d: web::Data<AppState>, path: web::Path<i32>, body: web::Json<UpdateEntryData>) -> impl Responder {
    let id = path.into_inner();     //pulls value from path

    match sqlx::query_as!(
        Todo,
        "UPDATE todolist SET username = $1, description = $2 WHERE id = $3 RETURNING id, username, description, time, date",
        body.username.to_string(),
        body.description.to_string(),
        id
    )
        .fetch_one(&d.db)
        .await
    {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user article"),
    }
}

#[delete("/todolist/data/{id}")]
async fn delete_entry(d: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query_as!(
        Todo,
        "DELETE FROM todolist WHERE id = $1 RETURNING id, username, description, time, date",
        id
    )
        .fetch_all(&d.db)
        .await
    {
        Ok(todolist) => HttpResponse::Ok().json(todolist),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete"),
    }

}
