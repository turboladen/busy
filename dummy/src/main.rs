use busy::{
    busy_error::StdBusyError, Application, BusyMethod, BusyRequest, BusyResponse, StatusCode,
    routing::Pipeline,
};
use futures::{future, Future};
// This needs to go away
use hyper::{Body, Response};

static TEMPLATE: &[u8] = b"<html><body><h1>you did it</h1></body></html>";

struct DummyApp;

enum Pipelines {
    Browser,
    Api
}

impl Pipeline for Pipelines {
    fn pipeline(&self) {
        match self {
            Pipelines::Browser => {},
            Pipelines::Api => {}
        }
    }
}

impl Application for DummyApp {
    // fn route(
    //     request: BusyRequest,
    // ) -> Box<Future<Item = BusyResponse, Error = StdBusyError> + Send> {
    //     match (request.method(), request.uri().path()) {
    //         (&BusyMethod::GET, "/") => Box::new(future::ok(Response::new(TEMPLATE.into()))),
    //         // This should be handled by the framework
    //         _ => Box::new(future::ok(
    //             Response::builder()
    //                 .status(StatusCode::NOT_FOUND)
    //                 .body(Body::empty())
    //                 .unwrap(),
    //         )),
    //     }
    // }
    fn route() {
        Router::new()
            .pipeline(
    }
}

fn main() {
    DummyApp::start()
}
