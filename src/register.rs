use crate::structs::*;

use axum::extract::State;
use axum::response::Html;
use axum::{
    Form, response::Redirect, extract::Query
};

pub async fn send_register(Query(q): Query<ErrorQuery>) -> Html<String> {
    match q.error.as_deref() {
        Some("password_missmatch") => {
            match std::env::var("REGISTER_ERROR_PASSWORDS_PATH") {
                Ok(data) => {
                    match std::fs::read_to_string(data) {
                        Ok(body) => return Html(body),
                        Err(_) => return Html("<p>ERROR</p>".to_string()),
                    }
                }
                Err(_) => return Html("<p>ERROR</p>".to_string())
            }
        }

        Some("hash_fail") => {
            match std::env::var("REGISTER_ERROR_HASH_PATH") {
                Ok(data) => {
                    match std::fs::read_to_string(data) {
                        Ok(body) => return Html(body),
                        Err(_) => return Html("<p>ERROR</p>".to_string()),
                    }
                }
                Err(_) => return Html("<p>ERROR</p>".to_string())
            }
        }
    
        Some("exists") => {
            match std::env::var("REGISTER_ERROR_EXISTS_PATH") {
                Ok(data) => {
                    match std::fs::read_to_string(data) {
                        Ok(body) => return Html(body),
                        Err(_) => return Html("<p>ERROR</p>".to_string()),
                    }
                }
                Err(_) => return Html("<p>ERROR</p>".to_string())
            }
        }

        Some("db") => {
            match std::env::var("REGISTER_ERROR_DB_PATH") {
                Ok(data) => {
                    match std::fs::read_to_string(data) {
                        Ok(body) => return Html(body),
                        Err(_) => return Html("<p>ERROR</p>".to_string()),
                    }
                }
                Err(_) => return Html("<p>ERROR</p>".to_string())
            }
        }

        Some("unknown") => {
            match std::env::var("REGISTER_ERROR_UNKNOWN_PATH") {
                Ok(data) => {
                    match std::fs::read_to_string(data) {
                        Ok(body) => return Html(body),
                        Err(_) => return Html("<p>ERROR</p>".to_string()),
                    }
                }
                Err(_) => return Html("<p>ERROR</p>".to_string())
            }
        }

         _ => {
            match std::env::var("REGISTER_PATH") {
                Ok(data) => {
                    match std::fs::read_to_string(data) {
                        Ok(body) => return Html(body),
                        Err(_) => return Html("<p>ERROR</p>".to_string())
                    }
                }
                Err(_) => return Html("<p>ERROR</p>".to_string())
            }
        }
    }
}


pub async fn register_submit( State(UserPool(pool)): State<UserPool>, Form(data): Form<RegisterData> ) -> Redirect {
    if data.password != data.confirm_password {
        return Redirect::to("/register?error=password_missmatch");
    }
    
    let hashed = match bcrypt::hash(&data.password, 10) {
        Ok(h) => h,
        Err(_) => return Redirect::to("/register?error=hash_fail"),
    };

    let try_insert_user: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> = sqlx::query(
        r#"
        INSERT INTO users (email, password)
        VALUES (?, ?)
        "#
    )
    .bind(&data.email)
    .bind(&hashed)
    .execute(&pool)
    .await;

    match try_insert_user {
        Ok(_) => return Redirect::to("/login"),
            
        Err(sqlx::Error::Database(db_err)) => {
            match db_err.code().as_deref() {
                Some("2067") => return Redirect::to("/register?error=exists"),
                _ => return Redirect::to("/register?error=db"),
            }
        }

        Err(_) => return Redirect::to("/register?error=unknown"),
    }
}
