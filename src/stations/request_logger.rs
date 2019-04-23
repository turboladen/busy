use crate::error::Error;
use busy_conveyor::{connect::Connect, connection::Connection};

#[derive(Clone, Copy)]
pub struct RequestLogger;

impl RequestLogger {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for RequestLogger {
    fn default() -> Self {
        pretty_env_logger::try_init().ok();

        Self
    }
}

impl Connect for RequestLogger {
    type Error = Error;
    type Params = Option<()>;

    #[inline]
    fn connect(
        &self,
        connection: Connection,
        _params: Self::Params,
    ) -> Result<Connection, Self::Error> {
        let request = connection.request();

        debug!(
            "[-> {:?} {} {}]",
            request.version(),
            request.method(),
            request.uri()
        );

        Ok(connection)
    }
}
