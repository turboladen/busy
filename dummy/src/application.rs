use crate::controllers::home;
use busy::{
    application::{Accept, Conn, Route, REQUEST_LOGGER},
    conveyor::{Connect, Connection},
    hyper::Body,
    router::Router,
    Error, Application,
};
use futures::{Future, Stream};

fn read_body_to_string(body: &mut Body) -> Result<String, Error> {
    body
        .concat2().map_err(Error::from).map(|chunk| {
            let v = chunk.to_vec();
            String::from_utf8_lossy(&v).to_string()
        })
        .wait()
}

pub(crate) struct BlogApp;

impl<T> Accept<T> for BlogApp {
    fn accept(conn: &mut Conn<T>, body: &mut Body) -> Result<(), Error> {
        REQUEST_LOGGER.connect(conn, None);
        let body = read_body_to_string(body)?;
        conn.request_body = Some(body);
    }
}

impl<T> Route<T> for BlogApp {
    fn route(conn: &mut Conn<T>) -> Result<(), Error> {
        let mut router = Router::default();

        router.get("/", home::index);

        router.route(conn);
    }
}

impl<T> Application<T> for BlogApp
where
    T: 'static + Send + Sync
{
}
