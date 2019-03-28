use crate::stations::logger::Logger;
use crate::{busy_error::StdBusyError, configuration::Configuration};
use busy_conveyor::{connection::Connection, station::Station};
use futures::Future;
use hyper::{service::service_fn, Body, Request, Response, Server};
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
    fn start()
    where
        Self: 'static,
    {
        dbg!(&*CONFIG);

        let server = Server::bind(&CONFIG.host)
            .serve(|| {
                service_fn(|req: Request<Body>| {
                    let connection = Connection::new(req);
                    let connection = LOGGER.operate(connection);
                    Self::route(connection)
                })
            })
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }

    fn route(
        connection: Connection,
    ) -> Box<Future<Item = Response<Body>, Error = StdBusyError> + Send>;
}
