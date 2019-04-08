use busy::{
    // busy_error::StdBusyError,
    application::LOGGER,
    conveyor::{connection::Connection, connect::Connect},
    router::{EasyRoute, Router},
    HyperApplication,
};
use crate::controllers::home;

pub(crate) struct BlogApp;

impl HyperApplication for BlogApp {
    type RouteResult = EasyRoute;

    fn route(connection: Connection) -> Self::RouteResult {
        let mut router = Router::default();

        router.get("/", home::index);

        router.route(connection)
    }

    fn endpoint(connection: Connection) -> Connection {
        // Static resources
        // Request ID
        LOGGER.connect(connection)
        // Parsers
        // Method override
        // Head
        // Session
        // Router
    }
}
