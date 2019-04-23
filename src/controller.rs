use crate::error::Error;
use busy_conveyor::connection::Connection;
use hyper::Body;

// pub fn render<B>(connection: Connection, body: B)
//     where B: Into<Body>
// {
// }
pub fn text<B>(connection: Connection, body: B) -> Result<Connection, Error>
where
    B: Into<Body>,
{
    let mut temp_connection = Connection { ..connection };

    temp_connection
        .response_builder
        .header("Content-Type", "text/plain");

    Ok(Connection {
        response_body: Some(body.into()),
        ..temp_connection
    })
}
