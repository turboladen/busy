use crate::{connection::Connection, error::StdError};
use futures::Future;
use hyper::{Body, Response};

pub trait Station {
    fn operate(self, connection: Connection) -> Connection;
}

pub trait FinalStation {
    fn operate(
        self,
        connection: Connection,
    ) -> Box<Future<Item = Response<Body>, Error = StdError> + Send>;
}
