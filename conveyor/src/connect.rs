use crate::connection::Connection;

pub trait Connect {
    type Params;
    type Error;

    fn connect(&self, connection: Connection, params: Self::Params) -> Result<Connection, Self::Error>;
}
