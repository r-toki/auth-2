use crate::model::{
    lib::jwt::Tokens,
    user::{SignIn, SignUp, User},
};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Json, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
    cfg.service(create_sessions);
}

#[derive(Debug, Deserialize)]
struct Create {
    name: String,
    password: String,
}

#[post("/users")]
async fn create(conn: Data<PgPool>, form: Json<Create>) -> actix_web::Result<Json<Tokens>> {
    User::sign_up(
        &conn,
        SignUp::new(form.name.to_owned(), form.password.to_owned()),
    )
    .await
    .map(Json)
    .map_err(ErrorInternalServerError)
}

#[derive(Debug, Deserialize)]
struct CreateSessions {
    name: String,
    password: String,
}

#[post("/users/sessions")]
async fn create_sessions(
    conn: Data<PgPool>,
    form: Json<CreateSessions>,
) -> actix_web::Result<Json<Tokens>> {
    User::sign_in(
        &conn,
        SignIn::new(form.name.to_owned(), form.password.to_owned()),
    )
    .await
    .map(Json)
    .map_err(ErrorInternalServerError)
}
