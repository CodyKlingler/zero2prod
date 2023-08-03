use actix_web::{Responder, HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    if form.name.is_empty() || form.email.is_empty() {
        HttpResponse::BadRequest()
    }
    else {
        HttpResponse::Ok()
    }
}
