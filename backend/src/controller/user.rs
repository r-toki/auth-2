use super::lib::{error::Result, image::Image, jwt_extractor::AccessTokenDecoded};
use crate::model::{
    attachment::Attachment,
    blob::Blob,
    user::{User, UserDto},
};

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
async fn update(
    pool: Data<PgPool>,
    access_token_decoded: AccessTokenDecoded,
    form: Json<Update>,
) -> Result<Json<()>> {
    let auth = access_token_decoded.into_auth();

    let attachment = Attachment::find_by_record(
        &**pool,
        "users".into(),
        auth.user_id.clone(),
        "image".into(),
    )
    .await?
    .into_iter()
    .nth(0);

    let blob = Blob::create(
        form.image.encoded.clone(),
        form.image.name.clone(),
        form.image.content_type.clone(),
    )?;

    match attachment {
        Some(mut attachment) => {
            attachment.id = blob.id.clone();
            let mut tx = pool.begin().await?;
            attachment.upsert(&mut tx).await?;
            blob.upsert(&mut tx).await?;
        }

        None => {
            let attachment = Attachment::create(
                "users".into(),
                auth.user_id.clone(),
                "image".into(),
                blob.id.clone(),
            );
            let mut tx = pool.begin().await?;
            attachment.upsert(&mut tx).await?;
            blob.upsert(&mut tx).await?;
        }
    }

    Ok(Json(()))
}
