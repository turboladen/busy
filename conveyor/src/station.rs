use crate::connection::Connection;

pub trait Station {
    fn operate(self, connection: Connection) -> Connection;
}
