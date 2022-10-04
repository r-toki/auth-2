mod lib;
mod user;
mod users;

use actix_web::web::ServiceConfig;

pub fn init(cfg: &mut ServiceConfig) {
    user::init(cfg);
    users::init(cfg);
}
