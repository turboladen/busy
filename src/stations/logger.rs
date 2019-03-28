use busy_conveyor::{connection::Connection, station::Station};

#[derive(Clone, Copy)]
pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        pretty_env_logger::try_init().ok();

        Self
    }
}

impl Station for Logger {
    fn operate(self, connection: Connection) -> Connection {
        let request = connection.request();

        debug!("[{} {}]", request.method(), request.uri());

        connection
    }
}
