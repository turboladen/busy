mod application;
mod controllers;

use busy::Application;

fn main() {
    crate::application::BlogApp.start()
}
