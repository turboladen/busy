use busy_conveyor::error::Error as ConveyorError;
use failure::{Compat, Fail};
use hyper::http::Error as HttpError;
use hyper::Error as HyperError;

pub type StdError = Compat<Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "internal http error: {}", _0)]
    HttpError(#[cause] HttpError),

    #[fail(display = "internal hyper error: {}", _0)]
    HyperError(#[cause] HyperError),

    #[fail(display = "busy_conveyor error: {}", _0)]
    ConveyorError(#[cause] ConveyorError),
}

impl From<ConveyorError> for Error {
    fn from(conveyor_error: ConveyorError) -> Self {
        Error::ConveyorError(conveyor_error)
    }
}

impl From<HyperError> for Error {
    fn from(hyper_error: HyperError) -> Self {
        Error::HyperError(hyper_error)
    }
}

impl From<HttpError> for Error {
    fn from(http_error: HttpError) -> Self {
        Error::HttpError(http_error)
    }
}
