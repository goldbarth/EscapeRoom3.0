use crate::application::Application;
use crossterm::{
    ExecutableCommand, execute,
    cursor::Hide
};

mod application;
mod menu;
mod console;

fn main() {
    execute!(std::io::stdout(), Hide).unwrap();

    let mut app = Application::new();
    app.run();
}
