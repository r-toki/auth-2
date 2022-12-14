use super::config::CONFIG;

use actix_cors::Cors;
use actix_web::http;

pub fn cors() -> Cors {
    Cors::default()
        .allowed_origin(&CONFIG.frontend_origin)
        .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .max_age(3_600)
}
