use busy::{controller::text, conveyor::connection::Connection, router::Params, Error};

// This needs to go away
use hyper::Body;

pub(crate) fn index(
    connection: Connection,
    _params: Option<Params>,
) -> Result<Connection, Error> {
    static TEMPLATE: &[u8] = b"<html><body><h1>you did it</h1></body></html>";

    text(connection, Body::from(TEMPLATE))
}
