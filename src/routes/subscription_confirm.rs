use actix_web::HttpResponse;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber")]
pub async fn confirm() -> HttpResponse {
    HttpResponse::Ok().finish()
}
