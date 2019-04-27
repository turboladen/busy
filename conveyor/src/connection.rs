use crate::error::Error;
use futures::{self, Async, Future, Poll};
use http::{request::Parts, response::Builder};
use hyper::{Body, Request, Response};
use std::collections::HashMap;
use url::Url;

pub struct Connection {
    // pub request: Request<Body>,
    pub request_parts: Parts,
    pub request_body: Body,

    pub response_builder: Builder,
    pub response_body: Option<Body>,
}

impl Connection {
    pub fn new(request: Request<Body>) -> Self {
        let (parts, body) = request.into_parts();

        Self {
            request_parts: parts,
            request_body: body,
            response_builder: Response::builder(),
            response_body: None,
        }
    }

    pub fn request_parts(&self) -> &Parts {
        &self.request_parts
    }

    pub fn query_params(&self) -> Option<HashMap<String, String>> {
        let request_uri = self.request_parts.uri.to_string();

        let url = Url::parse(&request_uri).ok()?;

        let hash = url.query_pairs().fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k.into_owned(), v.into_owned());
            acc
        });

        Some(hash)
    }

    pub fn close(mut self) -> Result<Response<Body>, Error> {
        let body = match self.response_body {
            Some(b) => b,
            None => Body::empty(),
        };

        self.response_builder.body(body).map_err(Error::from)
    }
}

pub struct ConnectionFuture<F, E>
where
    F: Fn() -> Result<Connection, E>,
    E: From<Error>,
{
    connector: F,
}

impl<F, E> ConnectionFuture<F, E>
where
    F: Fn() -> Result<Connection, E>,
    E: From<Error>,
{
    pub fn new(connector: F) -> Self {
        Self { connector }
    }
}

impl<F, E> Future for ConnectionFuture<F, E>
where
    F: Fn() -> Result<Connection, E>,
    E: From<Error>,
{
    type Item = Connection;
    type Error = E;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match (self.connector)() {
            Ok(connection) => Ok(Async::Ready(connection)),
            Err(e) => Err(e),
        }
    }
}
