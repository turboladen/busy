use crate::busy_error::StdBusyError;
use futures::Future;
use hyper::{Body, Request, Response};

pub type BusyRoutedResponse = Box<Future<Item = Response<Body>, Error = StdBusyError> + Send>;

pub fn route(request: Request<Body>) -> BusyRoutedResponse {
    unimplemented!();
}

pub trait Routable {
    fn route(
        request: Request<Body>,
    ) -> Box<Future<Item = Response<Body>, Error = StdBusyError> + Send>;

    fn _route(request: Request<Body>) -> Box<Future<Item = Response<Body>, Error = StdBusyError> + Send> {

    }
}

pub struct Router {
    pipelines: Vec<Pipeline>,
}

impl Router {
    pub fn new() -> Self {
        Router { pipelines: vec![] }
    }

    pub fn pipeline<'a, F>(&'a mut self, f: F) -> &'a mut Self
        where F: FnOnce() -> Pipeline
    {
        self.pipelines.push(f());
        self
    }
}

pub trait Pipeline {
    fn pipeline(&self, request: Request<Body>);
}
