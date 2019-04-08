use crate::{connection::Connection, error::Error};
use futures::Future;

pub trait Connect {
    fn connect(&self, connection: Connection) -> Connection;
}

pub trait AsyncConnect {
    fn async_connect(&self, connection: Connection) -> dyn Future<Item = Connection, Error = Error>;
}
