use crate::{busy_error::StdBusyError, configuration::Configuration};
use futures::Future;
use hyper::{service::service_fn, Body, Request, Response, Server};

pub trait Application {
    fn start()
    where
        Self: 'static,
    {
        let config = Self::build_configuration();
        dbg!(&config);

        let server = Server::bind(&config.host)
            .serve(|| service_fn(Self::route))
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }

    fn build_configuration() -> Configuration {
        Configuration::try_new().expect("Unable to fetch configuration")
    }

    fn route(
        request: Request<Body>,
    ) -> Box<Future<Item = Response<Body>, Error = StdBusyError> + Send>;
}
