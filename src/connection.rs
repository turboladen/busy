pub use hyper::{Request, Body};

pub struct Connection {
    request: Request<Body>,
}

impl Connection {
    pub fn new(request: Request<Body>) -> Self {
        Self {
            request
        }
    }

    pub fn request(&self) -> &Request<Body> {
        &self.request
    }
}
