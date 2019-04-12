use crate::busy_error::BusyError;
use busy_conveyor::connection::Connection;
use hyper::{Method, StatusCode};
use std::collections::HashMap;

pub type Params = HashMap<String, String>;

pub type Action = fn(Connection, Option<Params>) -> Result<Connection, BusyError>;

pub struct Route {
    method: Method,
    path: String,
    action: Action,
}

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn get<'a>(&'a mut self, path: &str, action: Action) -> &'a Self {
        let route = Route {
            method: Method::GET,
            action,
            path: path.to_string(),
        };
        self.routes.push(route);
        self
    }

    #[inline]
    pub fn route(&self, connection: Connection) -> Result<Connection, BusyError> {
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

        Ok(Connection { ..temp_connection })
    }
}

impl Default for Router {
    fn default() -> Self {
        Self { routes: vec![] }
    }
}
