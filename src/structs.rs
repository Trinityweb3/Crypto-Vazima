use serde::Deserialize;
use sqlx::{Pool, Sqlite};
use axum::extract::FromRef;
#[derive(Deserialize)]
pub struct ErrorQuery {
    pub error: Option<String>,
}


#[derive(Clone)]
pub struct UserPool(pub Pool<Sqlite>);


#[derive(Clone)]
pub struct VacancyPool(pub Pool<Sqlite>);


#[derive(Clone, FromRef)]
pub struct AppState {
    pub users_pool: UserPool,
    pub vacansies_pool: VacancyPool
}


#[derive(Deserialize)]
pub struct RegisterData {
    pub email: String,
    pub password: String,
    pub confirm_password: String
}



#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct Vacansy {
    pub title: String,
    pub body: String,
    pub link: String,
    pub min_salary: i32,
    pub max_salary: i32,
}



#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct DeleteRequire {
    pub title: String
}
