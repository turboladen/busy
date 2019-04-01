use crate::stations::logger::Logger;
use crate::{
    busy_error::{BusyError, StdBusyError},
    configuration::Configuration,
};
use busy_conveyor::{connection::Connection, station::Station};
use failure::Fail;
use futures::Future;
use hyper::{service::service_fn, Body, Request, Server};
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref CONFIG: Configuration =
        Configuration::try_new().expect("Unable to fetch configuration");
    static ref LOGGER: Logger = {
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", format!("busy={}", CONFIG.log_level.to_string()));
        }

        Logger::new()
    };
}

pub trait HyperApplication {
    type RouteResult: Future<Item = Connection, Error = StdBusyError> + Send;

    fn start()
    where
        Self: 'static,
    {
        dbg!(&*CONFIG);

        let server = Server::bind(&CONFIG.host)
            .serve(|| {
                service_fn(|req: Request<Body>| {
                    let connection = Connection::new(req);

                    // pre-routing things. Make synchronous for now.
                    let connection = LOGGER.operate(connection);

                    // Hand the connection over to the router, where each route must return a
                    // future that contains the not-yet-completed response (since response is
                    // completed by calling Builder::body()). Then here we'll finalize the response
                    // body in a Future and hand it back over to hyper.

                    Self::route(connection).and_then(|connection| {
                        Box::new(connection.close().map_err(|e| BusyError::from(e).compat()))
                    })
                })
            })
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }

    fn route(connection: Connection) -> Self::RouteResult;
}
