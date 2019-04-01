use crate::error::Error;
use futures::{Future, future::FutureResult};
use hyper::{Body, Request, Response};
use http::response::Builder;
use url::Url;
use std::collections::HashMap;

pub struct Connection {
    pub request: Request<Body>,

    pub response_builder: Builder,
    pub response_body: Option<Body>,
}

impl Connection {
    pub fn new(request: Request<Body>) -> Self {
        Self {
            request,
            response_builder: Response::builder(),
            response_body: None,
        }
    }

    pub fn request(&self) -> &Request<Body> {
        &self.request
    }

    pub fn query_params<'a>(&self) -> Option<HashMap<String, String>> {
        let request_uri = self.request.uri().to_string();

        let url = Url::parse(&request_uri)
            .ok()?;

        let hash = url.query_pairs()
            .into_iter()
            .fold(HashMap::new(), |mut acc, (k, v)| {
                acc.insert(k.into_owned(), v.into_owned());
                acc
            });

        Some(hash)
    }

    pub fn close(mut self) -> impl Future<Item = Response<Body>, Error = Error> {
        let body = match self.response_body {
            Some(b) => b,
            None => Body::empty(),
        };

        FutureResult::from(
            self.response_builder.body(body)
            .map_err(|e| Error::from(e))
        )
    }
}
