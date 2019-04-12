use crate::{connection::Connection, error::Error};
use futures::Future;
use std::collections::HashMap;

pub trait Connect {
    type Params;
    type Error;

    fn connect(&self, connection: Connection, params: Self::Params) -> Result<Connection, Self::Error>;
}
