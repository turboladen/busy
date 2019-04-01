use crate::busy_error::StdBusyError;
use busy_conveyor::connection::Connection;
use futures::{future, Future};
use hyper::{Method, StatusCode};
use std::collections::HashMap;

pub type EasyRoute = Box<Future<Item = Connection, Error = StdBusyError> + Send>;

pub type Params = HashMap<String, String>;

pub type Action = fn(Connection, Option<Params>) -> EasyRoute;

pub struct Route {
    method: Method,
    path: String,
    action: Action,
}

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn add_route<'a>(&'a mut self, method: Method, path: &str, action: Action) -> &'a Self {
        let route = Route {
            method,
            action,
            path: path.to_string(),
        };
        self.routes.push(route);
        self
    }

    pub fn route(
        &self,
        connection: Connection,
    ) -> EasyRoute {
        let request = connection.request();

        for route in &self.routes {
            if (request.method(), request.uri().path()) == (&route.method, &route.path) {
                let query_params = connection.query_params();

                return (route.action)(connection, query_params);
            }
        }

        let mut temp_connection = Connection { ..connection };

        temp_connection
            .response_builder
            .status(StatusCode::NOT_FOUND);

        Box::new(future::ok(Connection { ..temp_connection }))
    }
}

impl Default for Router {
    fn default() -> Self {
        Self { routes: vec![] }
    }
}
