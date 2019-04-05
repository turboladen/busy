use busy::{
    // busy_error::StdBusyError,
    conveyor::connection::Connection,
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
}
