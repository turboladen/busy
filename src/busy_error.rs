use busy_conveyor::error::Error as ConveyorError;
use failure::{Compat, Fail};
use hyper::Error as HyperError;

pub type StdBusyError = Compat<BusyError>;

#[derive(Debug, Fail)]
pub enum BusyError {
    #[fail(display = "internal hyper error: {}", _0)]
    HyperError(#[cause] HyperError),

    #[fail(display = "busy_conveyor error: {}", _0)]
    ConveyorError(#[cause] ConveyorError),
}

impl From<ConveyorError> for BusyError {
    fn from(conveyor_error: ConveyorError) -> Self {
        BusyError::ConveyorError(conveyor_error)
    }
}

impl From<HyperError> for BusyError {
    fn from(hyper_error: HyperError) -> Self {
        BusyError::HyperError(hyper_error)
    }
}
