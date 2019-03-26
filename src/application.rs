use crate::configuration::Configuration;
use futures::Future;
use hyper::{Body, Response, Server, service::service_fn_ok};

pub trait Application {
    fn start() {
        let config = Self::build_configuration();
        dbg!(&config);

        // And a NewService to handle each connection...
        let new_service = || {
            service_fn_ok(|_req| {
                Response::new(Body::from("Hello World"))
            })
        };

        let server = Server::bind(&config.host)
            .serve(new_service);

        hyper::rt::run(server.map_err(|e| {
            eprintln!("server error: {}", e);
        }));
    }

    fn build_configuration() -> Configuration {
        Configuration::try_new().expect("Unable to fetch configuration")
    }
}
