pub mod lib;
pub mod user;
pub mod users;

use actix_web::web::ServiceConfig;

pub fn init(cfg: &mut ServiceConfig) {
    users::init(cfg);
}
