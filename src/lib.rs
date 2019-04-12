#[macro_use]
extern crate log;

pub mod application;
pub mod busy_error;
mod configuration;
pub mod controller;
pub mod router;
pub mod stations;

pub use application::HyperApplication;
pub use busy_error::BusyError;

pub use hyper::Method as BusyMethod;
pub use hyper::StatusCode;

pub use busy_conveyor as conveyor;

use conveyor::Connection;

fn print_http_version(connection: Connection) -> Result<Connection, BusyError> {
    debug!("HTTP Version: {:?}", connection.request().version());

    Ok(connection)
}
