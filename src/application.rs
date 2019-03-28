use crate::{busy_error::StdBusyError, configuration::Configuration, routing::Routable};
use futures::Future;
use hyper::{service::service_fn, Server};

pub trait Application: Routable {
    fn start()
    where
        Self: 'static,
    {
        let config = Self::build_configuration();
        dbg!(&config);

        let server = Server::bind(&config.host)
            // .serve(|| service_fn(Self::route))
            .serve(|| service_fn(Self::_route))
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }

    fn build_configuration() -> Configuration {
        Configuration::try_new().expect("Unable to fetch configuration")
    }
}
