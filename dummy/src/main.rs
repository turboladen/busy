mod application;
mod controllers;

use busy::HyperApplication;

fn main() {
    crate::application::BlogApp::start()
}
