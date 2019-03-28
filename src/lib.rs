mod application;
pub mod busy_error;
mod configuration;
pub mod connection;

pub use application::HyperApplication;
pub use busy_error::BusyError;

pub type BusyResponse = hyper::Response<hyper::Body>;
pub type BusyRequest = hyper::Request<hyper::Body>;

pub use hyper::Method as BusyMethod;
pub use hyper::StatusCode;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
