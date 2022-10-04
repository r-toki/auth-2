#[derive(Debug)]
pub struct Error(anyhow::Error);

impl actix_web::ResponseError for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl<T: Into<anyhow::Error>> From<T> for Error {
    fn from(t: T) -> Self {
        Error(t.into())
    }
}

pub type Result<T> = actix_web::Result<T, Error>;
