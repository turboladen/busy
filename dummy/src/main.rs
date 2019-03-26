use busy::Application;

struct DummyApp;

impl Application for DummyApp {}

fn main() {
    DummyApp::start()
}
