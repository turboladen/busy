use busy_conveyor::{connection::Connection, station::Station};

#[derive(Clone, Copy)]
pub struct Logger;

impl Station for Logger {
    fn operate(self, connection: Connection) -> Connection {
        println!("Request URI: {}", connection.request().uri().to_string());
        connection
    }
}
