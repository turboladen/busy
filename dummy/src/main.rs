use busy::{
    busy_error::StdBusyError, HyperApplication, BusyMethod, BusyRequest, BusyResponse, StatusCode,
    connection::Connection,
};
use futures::{future, Future};
// This needs to go away
use hyper::{Body, Response};

static TEMPLATE: &[u8] = b"<html><body><h1>you did it</h1></body></html>";

struct DummyApp;

impl HyperApplication for DummyApp {
    fn route(
        connection: Connection<BusyRequest>,
    ) -> Box<Future<Item = BusyResponse, Error = StdBusyError> + Send> {
        let request = connection.request();

        match (request.method(), request.uri().path()) {
            (&BusyMethod::GET, "/") => Box::new(future::ok(Response::new(TEMPLATE.into()))),
            // This should be handled by the framework
            _ => Box::new(future::ok(
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap(),
            )),
        }
    }
}

fn main() {
    DummyApp::start()
}
