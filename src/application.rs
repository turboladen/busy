use crate::stations::request_logger::RequestLogger;
use crate::{
    busy_error::BusyError,
    configuration::Configuration,
};
use busy_conveyor::connection::Connection;
use failure::Fail;
use futures::Future;
use hyper::{service::service_fn, Body, Request, Server, Response};
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

use super::{print_http_version};

pub trait HyperApplication {
    fn start()
    where
        Self: 'static,
    {
        dbg!(&*CONFIG);

        let server = Server::bind(&CONFIG.host)
            .serve(|| {
                service_fn(|req: Request<Body>| {
                    let connection = Connection::new(req);

                    let connection = match default_connecting(connection) {
                        Ok(c) => c,
                        Err(_e) => {
                            return Response::builder()
                                .status(406)
                                .body(Body::empty())
                                .map_err(|e| BusyError::from(e).compat())
                        }
                    };

                    // pre-routing things. Make synchronous for now.

                    // Hand the connection over to the router, where each route must return a
                    // future that contains the not-yet-completed response (since response is
                    // completed by calling Builder::body()). Then here we'll finalize the response
                    // body in a Future and hand it back over to hyper.

                    Self::route(connection)
                        .map(|connection| connection.close())
                        .and_then(|response| {
                            response
                                .map_err(|e| BusyError::from(e))
                        })
                        .map_err(|e| e.compat())
                })
            })
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }


    fn endpoint(connection: Connection) -> Connection;
    fn route(connection: Connection) -> Result<Connection, BusyError>;
}

fn default_connecting(connection: Connection) -> Result<Connection, BusyError> {
    print_http_version(connection)
        .and_then(|connection| print_http_version(connection))
        .and_then(|connection| print_http_version(connection))
}
