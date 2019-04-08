use busy_conveyor::{connection::Connection, connect::Connect};

#[derive(Clone, Copy)]
pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Logger {
    fn default() -> Self {
        pretty_env_logger::try_init().ok();

        Self
    }
}

impl Connect for Logger {
    fn connect(&self, connection: Connection) -> Connection {
        let request = connection.request();

        debug!("[-> {:?} {} {}]", request.version(), request.method(), request.uri());

        connection
    }
}
