use crate::stations::request_logger::RequestLogger;
use crate::{error::Error, configuration::Configuration};
use failure::Fail;
use futures::{Async, Future, Poll};
use hyper::{service::service_fn, Body, Request, Response, Server};
use hyper::http::{request::Parts as RequestParts, response::Builder};
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref CONFIG: Configuration =
        Configuration::try_new().expect("Unable to fetch configuration");
    pub static ref REQUEST_LOGGER: RequestLogger = {
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", format!("busy={}", CONFIG.log_level.to_string()));
        }

        RequestLogger::new()
    };
}

pub trait Application<T>: Accept<T> + Route<T>
where T: 'static + Send + Sync
{
    fn start(&'static self) {
        dbg!(&*CONFIG);

        let server = Server::bind(&CONFIG.host)
            .serve(|| {
                service_fn(|req: Request<Body>| {
                    let (parts, body) = req.into_parts();
                    let mut conn = Conn::new(parts);

                    EndpointConnection::new(Self::accept, Self::route, &mut conn, body);

                    conn.close()
                        .map_err(|e| e.compat())
                })
            })
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }
}

type AcceptorFn<T> = fn(&mut Conn<T>, &mut Body) -> Result<(), Error>;

pub trait Accept<T> {
    fn accept(conn: &mut Conn<T>, request_body: &mut Body) -> Result<(), Error>;
}

type RouterFn<T> = fn(&mut Conn<T>) -> Result<(), Error>;

pub trait Route<T> {
    fn route(conn: &mut Conn<T>) -> Result<(), Error>;
}

pub struct Conn<T> {
    pub request_parts: RequestParts,
    pub request_body: Option<T>,
    pub response_builder: Builder,
    pub response_body: Option<Body>,
}

impl<T> Conn<T> {
    pub fn new(request_parts: RequestParts) -> Self {
        Self {
            request_parts,
            request_body: None,
            response_builder: Builder::new(),
            response_body: None,
        }
    }

    pub fn close(mut self) -> Result<Response<Body>, Error> {
        let body = match self.response_body {
            Some(b) => b,
            None => Body::empty(),
        };

        self.response_builder.body(body).map_err(Error::from)
    }
}

pub enum EndpointConnectionState {
    Accepting(Body),
    Routing,
    Done,
}

// This should really be the `Connection`. Leaving as this name until I can prove this out.
pub struct EndpointConnection<'a, T>
where T: Send + Sync
{
    pub conn: &'a mut Conn<T>,
    pub state: EndpointConnectionState,
    pub accept_fn: AcceptorFn<T>,
    pub route_fn: RouterFn<T>,
}

impl<'a, T> EndpointConnection<'a, T>
where T: Send + Sync
{
    pub fn new(
        accept_fn: AcceptorFn<T>,
        route_fn: RouterFn<T>,
        conn: &'a mut Conn<T>,
        body: Body,
    ) -> Self {

        Self {
            accept_fn,
            route_fn,
            conn,
            state: EndpointConnectionState::Accepting(body),
        }
    }
}

impl<'a, T> Future for EndpointConnection<'a, T>
where T: Send + Sync
{
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match &mut self.state {
                EndpointConnectionState::Accepting(ref mut request_body) => {
                    match (self.accept_fn)(&mut self.conn, request_body) {
                        Ok(_) => {
                            self.state = EndpointConnectionState::Routing;
                        }
                        Err(e) => return Err(e)
                    }
                }
                EndpointConnectionState::Routing => {
                    match (self.route_fn)(&mut self.conn) {
                        Ok(_) => {
                            self.state = EndpointConnectionState::Done;
                        }
                        Err(e) => return Err(e)
                    }
                }
                EndpointConnectionState::Done => {
                    return Ok(Async::Ready(()));
                }
            }
        }
    }
}
