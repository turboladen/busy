use failure::{Compat, Fail};
use hyper::Error as HyperError;

pub type StdError = Compat<Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "internal hyper error: {}", _0)]
    HyperError(#[cause] HyperError),
}

impl From<HyperError> for Error {
    fn from(hyper_error: HyperError) -> Self {
        Error::HyperError(hyper_error)
    }
}
