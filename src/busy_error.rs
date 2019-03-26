use failure::{Compat, Fail};
use hyper::Error as HyperError;

pub type StdBusyError = Compat<BusyError>;

#[derive(Debug, Fail)]
pub enum BusyError {
    #[fail(display = "internal hyper error: {}", _0)]
    HyperError(#[cause] HyperError),
}

impl From<HyperError> for BusyError {
    fn from(hyper_error: HyperError) -> Self {
        BusyError::HyperError(hyper_error)
    }
}
