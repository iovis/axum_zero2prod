use axum::extract::State;
use axum::http::StatusCode;
use axum::Form;
use chrono::Utc;
use serde::Deserialize;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[axum::debug_handler]
pub async fn subscribe(State(db): State<SqlitePool>, Form(form): Form<FormData>) -> StatusCode {
    let id = Uuid::new_v4();
    let now = Utc::now();

    println!("uuid = {id}");

    let result = sqlx::query!(
        r#"
        insert into subscriptions (id, name, email, subscribed_at)
        values (?, ?, ?, ?);
        "#,
        id,
        form.name,
        form.email,
        now
    )
    .execute(&db)
    .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
