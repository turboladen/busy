use busy::{
    BusyError,
    controller::text,
    conveyor::connection::Connection,
    router::Params,
};

// This needs to go away
use hyper::Body;

pub(crate) fn index(
    connection: Connection,
    _params: Option<Params>,
) -> Result<Connection, BusyError> {
    static TEMPLATE: &[u8] = b"<html><body><h1>you did it</h1></body></html>";

    text(connection, Body::from(TEMPLATE))
}
