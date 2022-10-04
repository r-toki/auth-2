use super::lib::error::Result;
use crate::lib::jwt::Tokens;
use crate::model::user::User;
use actix_web::{
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
async fn create(pool: Data<PgPool>, form: Json<Create>) -> Result<Json<Tokens>> {
    let mut user = User::create(form.name.clone(), form.password.clone())?;
    let tokens = user.issue_tokens();
    user.upsert(&**pool).await?;
    Ok(Json(tokens))
}

#[derive(Debug, Deserialize)]
struct CreateSessions {
    name: String,
    password: String,
}

#[post("/users/sessions")]
async fn create_sessions(pool: Data<PgPool>, form: Json<CreateSessions>) -> Result<Json<Tokens>> {
    let mut user = User::find_by_name(&**pool, form.name.clone()).await?;
    user.verify_password(form.password.clone())?;
    let tokens = user.issue_tokens();
    user.upsert(&**pool).await?;
    Ok(Json(tokens))
}
