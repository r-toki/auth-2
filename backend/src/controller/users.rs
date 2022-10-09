use super::lib::{
    error::Result,
    jwt_extractor::{AccessTokenDecoded, BearerToken, RefreshTokenDecoded},
};
use crate::lib::jwt::Tokens;
use crate::model::user::User;

use actix_web::{
    delete, patch, post,
    web::{Data, Json, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
    cfg.service(create_sessions);
    cfg.service(update_sessions);
    cfg.service(delete_sessions);
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
    let mut user = User::find_by_name(&**pool, form.name.clone())
        .await?
        .ok_or_else(|| anyhow::anyhow!("not found"))?;
    user.verify_password(form.password.clone())?;
    let tokens = user.issue_tokens();
    user.upsert(&**pool).await?;
    Ok(Json(tokens))
}

#[patch("/users/sessions")]
async fn update_sessions(
    pool: Data<PgPool>,
    token: BearerToken,
    refresh_token_decoded: RefreshTokenDecoded,
) -> Result<Json<Tokens>> {
    let auth = refresh_token_decoded.into_auth();
    let mut user = User::find_by_id(&**pool, auth.user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("not found"))?;
    user.verify_refresh_token(token.into())?;
    let tokens = user.issue_tokens();
    user.upsert(&**pool).await?;
    Ok(Json(tokens))
}

#[delete("/users/sessions")]
async fn delete_sessions(
    pool: Data<PgPool>,
    access_token_decoded: AccessTokenDecoded,
) -> Result<Json<()>> {
    let auth = access_token_decoded.into_auth();
    let mut user = User::find_by_id(&**pool, auth.user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("not found"))?;
    user.revoke_tokens();
    user.upsert(&**pool).await?;
    Ok(Json(()))
}
