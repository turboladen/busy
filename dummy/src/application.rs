use busy::{
    // busy_error::StdBusyError,
    conveyor::connection::Connection,
    router::{EasyRoute, Router},
    BusyMethod, HyperApplication,
};
use crate::controllers::home;

pub(crate) struct BlogApp;

impl HyperApplication for BlogApp {
    type RouteResult = EasyRoute;

    fn route(connection: Connection) -> Self::RouteResult {
        Router::default()
            .add_route(BusyMethod::GET, "/", home::index)
            .route(connection)
    }
}
