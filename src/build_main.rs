use crate::structs::*;

use axum::extract::State;
use axum::response::Html;
use sqlx::Row;

pub async fn formating(text: String) -> String {
    let mut formatted_text: String = String::new();
    for c in text.chars() {
        match c {
            '<' => formatted_text.push_str("&lt;"),
            '>' => formatted_text.push_str("&gt;"),
            '&' => formatted_text.push_str("&amp;"),
            '"' => formatted_text.push_str("&quot;"),
            '\n' => formatted_text.push_str("<br>"),
            '\r' => continue,
            _ => formatted_text.push(c),
        }
    }
    return formatted_text;
}

pub async fn send_main(State(VacancyPool(pool)): State<VacancyPool>) -> Html<String> {
    let data = sqlx::query("SELECT * FROM vacansions")
        .fetch_all(&pool)
        .await;

    match data {
        Ok(rows) => {
            let main_base_path: String = std::env::var("MAIN_PATH_BASE").expect("Error reading main_base.html");
            let mut html_base: String = std::fs::read_to_string(main_base_path).expect("Error reading main_base.html");

            let main_end_path: String = std::env::var("MAIN_PATH_END").expect("Error reading main_end.html");
            let html_end: String = std::fs::read_to_string(main_end_path).expect("Error reading main_end.html");

            let debug_file_path: String = std::env::var("MAIN_PATH_DEBUG").unwrap();

            if rows.is_empty() {
                html_base.push_str("<p class='no_vacancies'>No vacansies available at the moment.</p>");
            } else {
                for row in rows {
                    let title: String = row.try_get("title").unwrap_or("Web3 Vacansion".to_string());
                    let body: String = row.try_get("body").unwrap_or("Apply to job".to_string());                   
                    let min_salary: i32 = row.try_get("min_salary").unwrap_or(0);
                    let max_salary: i32 = row.try_get("max_salary").unwrap_or(0);
                    let link: String= row.try_get("link").unwrap_or(String::from("https://t.me/crypto_vazima"));

                    let title: String = formating(title).await;
                    let body: String = formating(body).await;
                    let link = formating(link).await;

                    let text: String = format!(r#"
                        <div class="container">
                            <section class="jobs-container">
                                <article class="vacancy-card">
                                    <h2 class="vacancy-title">#{}</h2>
                                    <p class="vacancy-description">{}</p>
                                    <p class="vacancy-salary">Salary: ${}-${}</p>
                                    <a href="{}">
                                        <button class="vacancy-apply-btn">Apply Now</button>
                                    </a>
                                </article>
                            </section>
                        </div>
                    "#, title, body, min_salary, max_salary, link);

                    html_base.push_str(&text);
                }
            };
            html_base.push_str(&html_end);
            std::fs::write(debug_file_path, html_base.clone()).unwrap();
            return Html(html_base);
        },
        Err(_) => {
            let main_err_path: String = std::env::var("MAIN_PATH_ERR").expect("Error reading main_err.html");
            let html_err: String = std::fs::read_to_string(main_err_path).expect("Error reading main_err.html");
            return Html(html_err);
        }
    }
}
