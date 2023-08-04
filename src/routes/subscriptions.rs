use actix_web::{Responder, HttpResponse, web};
use uuid::Uuid;
use chrono::Utc;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    if form.name.is_empty() || form.email.is_empty() {
        return HttpResponse::BadRequest()
    }

    let db_response = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#, 
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await;

    match db_response {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => { 
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError()
        },
    }
        
}
