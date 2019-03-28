pub struct Connection<T> {
    request: T,
}

impl<T> Connection<T> {
    pub fn new(request: T) -> Self {
        Self {
            request
        }
    }

    pub fn request(&self) -> &T {
        &self.request
    }
}
