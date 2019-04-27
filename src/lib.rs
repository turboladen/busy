#![forbid(unused_imports)]
#![deny(unused_extern_crates)]
#![warn(
    box_pointers,
    future_incompatible,
    missing_copy_implementations,
    // missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications,
    clippy::all,
    // clippy::restriction,
    // clippy::pedantic,
    // clippy::nursery,
    // clippy::cargo,
)]

#[macro_use]
extern crate log;

pub mod application;
mod configuration;
pub mod controller;
pub mod error;
pub mod router;
pub mod stations;

pub use application::Application;
pub use error::Error;

pub use hyper::Method as BusyMethod;
pub use hyper::StatusCode;

pub use busy_conveyor as conveyor;

use conveyor::Connection;

fn print_http_version(connection: Connection) -> Result<Connection, Error> {
    debug!("HTTP Version: {:?}", connection.request_parts().version);

    Ok(connection)
}
