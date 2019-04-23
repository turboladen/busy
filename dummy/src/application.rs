use crate::controllers::home;
use busy::{
    application::REQUEST_LOGGER,
    conveyor::{Connect, Connection},
    router::Router,
    Error, HyperApplication,
};

pub(crate) struct BlogApp;

impl HyperApplication for BlogApp {
    fn endpoint(connection: Connection) -> Result<Connection, Error> {
        REQUEST_LOGGER.connect(connection, None)
    }

    fn route(connection: Connection) -> Result<Connection, Error> {
        let mut router = Router::default();

        router.get("/", home::index);

        router.route(connection)
    }
}
