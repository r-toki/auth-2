mod lib;
mod user;
mod users;

use actix_web::web::ServiceConfig;

pub fn init(cfg: &mut ServiceConfig) {
    users::init(cfg);
}
