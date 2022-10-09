use super::lib::{error::Result, image::Image, jwt_extractor::AccessTokenDecoded};
use crate::model::user::{User, UserDto};

use actix_web::{
    get, patch,
    web::{Data, Json, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

#[get("/user")]
async fn index(
    pool: Data<PgPool>,
    access_token_decoded: AccessTokenDecoded,
) -> Result<Json<UserDto>> {
    let auth = access_token_decoded.into_auth();
    let me = User::find_user(&**pool, auth.user_id).await?;
    Ok(Json(me))
}

#[derive(Debug, Deserialize)]
struct Update {
    image: Image,
}

#[patch("/user")]
async fn update(pool: Data<PgPool>, access_token_decoded: AccessTokenDecoded) -> Result<Json<()>> {
    let auth = access_token_decoded.into_auth();
    Ok(Json(()))
}
