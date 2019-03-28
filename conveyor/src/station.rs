use crate::{connection::Connection, error::StdError};
use futures::Future;
use hyper::{Response, Body};

pub trait Station {
    fn operate(self, connection: Connection) -> Connection;
}

pub trait FinalStation {
    fn operation(self, connection: Connection) -> Box<Future<Item = Response<Body>, Error = StdError> + Send>;
}
