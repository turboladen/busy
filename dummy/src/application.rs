use busy::{
    application::REQUEST_LOGGER,
    conveyor::{Connection, Connect},
    router::{Router},
    HyperApplication,
    BusyError,
};
use crate::controllers::home;

pub(crate) struct BlogApp;

impl HyperApplication for BlogApp {
    fn endpoint(connection: Connection) -> Result<Connection, BusyError> {
        REQUEST_LOGGER.connect(connection, None)
    }

    fn route(connection: Connection) -> Result<Connection, BusyError> {
        let mut router = Router::default();

        router.get("/", home::index);

        router.route(connection)
    }
}
