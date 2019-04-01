use crate::busy_error::StdBusyError;
use busy_conveyor::connection::Connection;
use futures::{future, Future};
use hyper::Body;

// pub fn render<B>(connection: Connection, body: B)
//     where B: Into<Body>
// {
// }
pub fn text<B>(
    connection: Connection,
    body: B,
) -> Box<Future<Item = Connection, Error = StdBusyError> + Send>
where
    B: Into<Body>,
{
    let mut temp_connection = Connection { ..connection };

    temp_connection
        .response_builder
        .header("Content-Type", "text/plain");

    Box::new(future::ok(Connection {
        response_body: Some(body.into()),
        ..temp_connection
    }))
}
