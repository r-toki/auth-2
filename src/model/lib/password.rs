use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref PASSWORD_REGEX: Regex = Regex::new(r"[A-Za-z\d#$@!%&*?]{8,30}").unwrap();
}
