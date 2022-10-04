use super::lib::{error::Result, jwt_extractor::AccessTokenDecoded};
use crate::lib::jwt::Auth;
use crate::model::user::{User, UserDto};

use actix_web::{
    get,
    web::{Data, Json, ServiceConfig},
};
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

#[get("/user")]
async fn index(
    pool: Data<PgPool>,
    access_token_decoded: AccessTokenDecoded,
) -> Result<Json<UserDto>> {
    let auth: Auth = access_token_decoded.into();
    let me = User::find_user(&**pool, auth.sub).await?;
    Ok(Json(me))
}
