use crate::structs::*;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use regex::Regex;

use axum::extract::State;
use axum::response::Html;
use axum::{
    Form, response::Redirect, extract::Query
};

pub async fn login_submit( State(UserPool(pool)): State<UserPool>, Form(input): Form<LoginData>) -> Redirect {
    let email_input: String = input.email;
    if email_input.is_empty() {
        return Redirect::to("/login?error=email_required");
    } else {
        let needed_email_type = match Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$") {
            Ok(d) => d,
            Err(_) => return Redirect::to("/login?error=hashing_error")
        };
        
        let email_validation: bool = needed_email_type.is_match(&email_input);
        if email_validation == true {
            let password_input: &String = &input.password;
            let row= match sqlx::query("SELECT password FROM users WHERE email = ?")
                .bind(&email_input)
                .fetch_optional(&pool)
                .await {
                    Ok(d) => d,
                    Err(_) => return Redirect::to("/login?error=hashing_error")
                };

            let row: SqliteRow = match row {
                Some(r) => r,
                None => return Redirect::to("/login?error=user_not_found")
            };

            let hashed_password: String = row.get("password");

            let ok: bool = match bcrypt::verify(&password_input, &hashed_password) {
                    Ok(r) => r,
                    Err(_) => return Redirect::to("/login?error=hashing_error")
                };

            if ok == true {
                return Redirect::to("/post_vacansion_page");
            }
                else {
                    return Redirect::to("/login?error=incorrect_password");
            }

        } 
        else {
            return Redirect::to("/login?error=invalid_email");
        }
    }
}

pub async fn send_login(Query(q): Query<ErrorQuery>) -> Html<String> {
    match q.error.as_deref() {
        Some("invalid_email") => {
            match std::env::var("LOGIN_ERROR_INVALID_EMAIL") {
                Ok(body) => {
                    match std::fs::read_to_string(body) {
                        Ok(data) => return Html(data),
                        Err(_) => return Html("<p>ERROR</p>".to_string())
                    }
                }
                Err(_) => return Html("<p>ERROR</p>".to_string())
            }
        }

        Some("incorrect_password") => {
            match std::env::var("LOGIN_ERROR_INCORRECT_PASSWORD") {
                Ok(body) => {
                    match std::fs::read_to_string(body) {
                        Ok(data) => return Html(data),
                        Err(_) => return Html("<p>ERROR</p>".to_string())
                    }
                }
                Err(_) => return Html("<p>ERROR</p>".to_string())
            }
        }

        Some("hashing_error") => {
            match std::env::var("LOGIN_ERROR_HASH_PATH") {
                Ok(body) => {
                    match std::fs::read_to_string(body) {
                        Ok(data) => return Html(data),
                        Err(_) => return Html("<p>ERROR</p>".to_string())
                    }
                }
                Err(_) => return Html("<p>ERROR</p>".to_string())
            }
        }

        Some("user_not_found") => {
            match std::env::var("LOGIN_ERROR_USER_NOT_FOUND") {
                Ok(body) => {
                    match std::fs::read_to_string(body) {
                        Ok(data) => return Html(data),
                        Err(_) => return Html("<p>ERROR</p>".to_string())
                    }
                }
                Err(_) => return Html("<p>ERROR</p>".to_string())
            }
        }

        Some("email_required") => {
            match std::env::var("LOGIN_ERROR_EMAIL_REQUIRED") {
                Ok(body) => {
                    match std::fs::read_to_string(body) {
                        Ok(data) => return Html(data),
                        Err(_) => return Html("<p>ERROR</p>".to_string())
                    }
                }
                Err(_) => return Html("<p>ERROR</p>".to_string())
            }
        }

        _ => {
            match std::env::var("LOGIN_PATH") {
                Ok(body) => {
                    match std::fs::read_to_string(body) {
                        Ok(data) => return Html(data),
                        Err(_) => return Html("<p>ERROR</p>".to_string())
                    }
                }
                Err(_) => return Html("<p>ERROR</p>".to_string())
            }
        }
    }
}
