use failure::{Compat, Fail};
use http::Error as HttpError;
use hyper::Error as HyperError;

pub type StdError = Compat<Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "internal hyper error: {}", _0)]
    HyperError(#[cause] HyperError),

    #[fail(display = "internal http error: {}", _0)]
    HttpError(#[cause] HttpError),
}

impl From<HttpError> for Error {
    fn from(http_error: HttpError) -> Self {
        Error::HttpError(http_error)
    }
}

impl From<HyperError> for Error {
    fn from(hyper_error: HyperError) -> Self {
        Error::HyperError(hyper_error)
    }
}
