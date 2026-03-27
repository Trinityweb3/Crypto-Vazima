mod structs;
mod vacansies;
mod build_main;
mod login;
mod register;

use crate::register::*;
use crate::structs::*;
use crate::vacansies::*;
use crate::build_main::*;
use crate::login::*;

use std::env;

use axum::{
     Router, routing::{post, get}
};

use sqlx::{Pool, Sqlite, SqlitePool};


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_users_path: &str = &env::var("DATABASE_USERS_URL").unwrap();
    let database_vacansies_path: &str = &env::var("DATABASE_VACANSIES_URL").unwrap();

    let pool: Pool<Sqlite> = SqlitePool::connect(database_users_path).await.expect("db connect failed");
    let pool_vacancies: Pool<Sqlite> = SqlitePool::connect(database_vacansies_path).await.expect("db connect failed");

    let app_state = AppState {
        users_pool: UserPool(pool.clone()),
        vacansies_pool: VacancyPool(pool_vacancies.clone())
    };
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#
    )
    .execute(&pool)
    .await
    .expect("Init db table failed");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS vacansions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            link TEXT NOT NULL,
            min_salary INTEGER,
            max_salary INTEGER
        );
        "#
    )
    .execute(&pool_vacancies)
    .await
    .expect("Init db table failed");

    let app = Router::new()
        .route("/", get(send_main))
        .route("/register", get(send_register))
        .route("/register_submit", post(register_submit))
        .route("/login", get(send_login))
        .route("/login_submit", post(login_submit))
        .route("/post_vacansion_page", get(send_vacansion))
        .route("/post_vacansion", post(add_vacansion))
        .route("/delete_vacansion", post(delete_vacansion))
        .route("/send_delete_vacansion_form", get(send_delete_vacansion_form))
        .with_state(app_state);



    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("listener error");
    axum::serve(listener, app).await.expect("axum::serve error")
}