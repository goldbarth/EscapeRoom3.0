use crate::application::Application;

mod application;
mod menu;
mod console;

fn main() {
    let mut app = Application::new();
    app.run();
}
