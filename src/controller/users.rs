use actix_web::{
    post,
    web::{Json, ServiceConfig},
    Responder,
};
use serde::{Deserialize, Serialize};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
}

#[derive(Debug, Deserialize)]
struct Create {
    name: String,
    password: String,
}

#[post("/users")]
async fn create(form: Json<Create>) -> impl Responder {
    Json(())
}
