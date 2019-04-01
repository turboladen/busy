use busy::{
    busy_error::StdBusyError,
    controller::text,
    conveyor::connection::Connection,
    router::{EasyRoute, Params, Router},
    BusyMethod, HyperApplication,
};
// This needs to go away
use futures::Future;
use hyper::Body;

static TEMPLATE: &[u8] = b"<html><body><h1>you did it</h1></body></html>";

struct DummyApp;

impl HyperApplication for DummyApp {
    type RouteResult = EasyRoute;

    fn route(connection: Connection) -> Self::RouteResult {
        Router::default()
            .add_route(BusyMethod::GET, "/", home)
            .route(connection)
    }
}

fn home(
    connection: Connection,
    _params: Option<Params>,
) -> EasyRoute {
    text(connection, Body::from(TEMPLATE))
}

fn main() {
    DummyApp::start()
}
