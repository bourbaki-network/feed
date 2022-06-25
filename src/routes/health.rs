use actix_web::{get, HttpResponse, http::header::ContentType};
use serde::Serialize;


#[derive(Serialize)]
struct HealthAPIResponse {
    name: String,
    version: String,
}

#[get("/health")]
async fn health() -> HttpResponse {
    let ret = HealthAPIResponse{
        name: "feed-backend".to_owned(),
        version: "0.0.1".to_owned()
    };

    let j = serde_json::to_string(&ret);

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(j.unwrap())
}
