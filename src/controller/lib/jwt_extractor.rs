use crate::lib::jwt::{decode_access_token, decode_refresh_token, Auth, Claims};

use actix_web::{error::ErrorUnauthorized, http::header, FromRequest};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::future::Future;
use std::pin::Pin;

lazy_static! {
    static ref BEARER_REGEX: Regex = Regex::new(r"^Bearer\s(.*)$").unwrap();
}

pub struct BearerToken(String);

impl From<BearerToken> for String {
    fn from(v: BearerToken) -> Self {
        v.0
    }
}

impl FromRequest for BearerToken {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            extract_bearer_token(&req)
                .map(BearerToken)
                .map_err(ErrorUnauthorized)
        })
    }
}

pub struct AccessTokenDecoded(Claims);

impl From<AccessTokenDecoded> for Auth {
    fn from(v: AccessTokenDecoded) -> Self {
        Auth::new(v.0.sub)
    }
}

impl FromRequest for AccessTokenDecoded {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            extract_bearer_token(&req)
                .and_then(|token| decode_access_token(&token))
                .map(AccessTokenDecoded)
                .map_err(ErrorUnauthorized)
        })
    }
}

pub struct RefreshTokenDecoded(Claims);

impl From<RefreshTokenDecoded> for Auth {
    fn from(v: RefreshTokenDecoded) -> Self {
        Auth::new(v.0.sub)
    }
}

impl FromRequest for RefreshTokenDecoded {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            extract_bearer_token(&req)
                .and_then(|token| decode_refresh_token(&token))
                .map(RefreshTokenDecoded)
                .map_err(ErrorUnauthorized)
        })
    }
}

fn extract_bearer_token(req: &actix_web::HttpRequest) -> anyhow::Result<String> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|authorization| {
            BEARER_REGEX
                .captures(authorization)
                .and_then(|captures| captures.get(1))
        })
        .map(|v| v.as_str().to_owned())
        .ok_or_else(|| anyhow::anyhow!("invalid authorization method"))
}
