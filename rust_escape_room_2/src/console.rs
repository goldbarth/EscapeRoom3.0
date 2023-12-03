use crossterm::{execute, cursor, terminal::{Clear, ClearType}};
use std::io;

pub fn set_cursor_position(x: u16, y: u16) {
    execute!(io::stdout(), cursor::MoveTo(x, y)).expect("Failed to set cursor position");
}

pub fn clear_console() {
    Clear(ClearType::All);
}