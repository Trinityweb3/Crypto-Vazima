use crate::structs::*;

use axum::extract::State;
use axum::response::Html;
use axum::{
    Form, response::Redirect, extract::Query
};

pub async fn send_vacansion(Query(q): Query<ErrorQuery>) -> Html<String> {
    match q.error.as_deref() {
        Some("error_db") => {
            match std::env::var("VACANSION_ADD_FORM_PATH_ERROR_DB") {
                Ok(data) => {
                    match std::fs::read_to_string(data) {
                        Ok(body) => return Html(body),
                        Err(_) => return Html("<p>ERROR</p>".to_string())
                    }
                }
                Err(_) => return Html("<p>ERROR</p>".to_string())
            }
        }            
        Some("error_input") => {
            match std::env::var("VACANSION_ADD_FORM_PATH_ERROR_INPUT") {
                Ok(data) => {
                    match std::fs::read_to_string(data) {
                        Ok(body) => return Html(body),
                        Err(_) => return Html("<p>ERROR</p>".to_string())
                    }
                }
                Err(_) => return Html("<p>ERROR</p>".to_string())
            }
        }
        _ => { 
            match std::env::var("VACANSION_ADD_FORM") {
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

pub async fn delete_vacansion(State(VacancyPool(pool)): State<VacancyPool>, Form(input): Form<DeleteRequire>) -> Redirect {
    let title: String = input.title;
    let try_delete = sqlx::query(r#"DELETE FROM vacansions WHERE title = (?)"#)
        .bind(title)
        .execute(&pool)
        .await;

    match try_delete {
        Ok(_) => {
            return Redirect::to("/");
        },
        Err(_) => {
            return Redirect::to("/send_delete_vacansion_form?error=delete_error");
        }
    }
}

pub async fn add_vacansion(State(VacancyPool(pool)): State<VacancyPool>, Form(data): Form<Vacansy>) -> Redirect {
    if data.max_salary > data.min_salary {
        let title: String = data.title;
        let min_salary: i32 = data.min_salary;
        let max_salary: i32 = data.max_salary;
        let body: String = data.body;
        let link: String = data.link;

        let try_insert_vacansion= sqlx::query (
            r#"
            INSERT INTO vacansions (title, body, link, min_salary, max_salary)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(title)
        .bind(body)
        .bind(link)
        .bind(min_salary)
        .bind(max_salary)
        .execute(&pool)
        .await;

        match try_insert_vacansion {
            Ok(_) => {
                return Redirect::to("/");
            },
            Err(_) => {
                return Redirect::to("/post_vacansion_page?error=error_db");
            }
        }
    } else {
        return Redirect::to("/post_vacansion_page?error=error_input");
    }
    
}

pub async fn send_delete_vacansion_form(Query(q): Query<ErrorQuery>) -> Html<String> {
    match q.error.as_deref() {
        Some("delete_error") => {
            match std::env::var("VACANSION_DELETE_FORM_ERROR") {
                Ok(body) => {
                    match std::fs::read_to_string(body) {
                        Ok(data) => return Html(data),
                        Err(_) => return Html("<p>ERROR</p>".to_string())
                    }
                }
                Err(_) => {
                    return Html("<p>ERROR</p>".to_string())
                }
            }
        },
        _ => {
            match std::env::var("VACANSION_DELETE_FORM") {
                Ok(body) => {
                    match std::fs::read_to_string(body) {
                        Ok(data) => return Html(data),
                        Err(_) => return Html("<p>ERROR</p>".to_string())
                    }
                }
                Err(_) => return  Html("<p>ERROR</p>".to_string())
            }
        }
    }
}

