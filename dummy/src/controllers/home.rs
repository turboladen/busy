use busy::{
    controller::text,
    conveyor::connection::Connection,
    router::{EasyRoute, Params},
};

// This needs to go away
use hyper::Body;

pub(crate) fn index(
    connection: Connection,
    _params: Option<Params>,
) -> EasyRoute {
    static TEMPLATE: &[u8] = b"<html><body><h1>you did it</h1></body></html>";

    text(connection, Body::from(TEMPLATE))
}
