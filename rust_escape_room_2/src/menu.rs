use crate::console::clear_console;

use std::process::exit;
use std::io::{self, Write};
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{Clear, ClearType},
    cursor::MoveTo
};

pub struct Menu {
    options: Vec<&'static str>,
    initial_line: u16,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            options: vec!["Start Game", "Tutorial", "Exit"],
            initial_line: 0,
        }
    }

    pub fn initialize_main_menu(&self) {
        draw_title_screen();

        let current_line = self.initial_line + 21;
        self.draw_game_options(current_line);
    }

    pub fn initialize_tutorial(&self) {
        write!(io::stdout(), "Tutorial").expect("TODO: panic message");
        let current_line = self.initial_line + 16;
        self.draw_game_options(current_line);
    }

    pub fn initialize_outro(&self) {
        write!(io::stdout(), "Outro").expect("TODO: panic message");
        let current_line = self.initial_line + 16;
        self.draw_game_options(current_line);
    }

    pub fn initialize_exit(&self) {
        clear_console();
        exit(0);
    }

    fn draw_game_options(&self, mut current_line: u16) {
        let num_options = self.options.len()  as u16;

        let mut current_option = 0u16;
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        loop {
            handle.execute(Clear(ClearType::CurrentLine)).unwrap();
            handle.execute(MoveTo(0, current_line)).unwrap();

            // Draw the menu options with a > in front of the selected option
            for i in 0..num_options {
                if i == current_option {
                    println!("> {}", self.options[i as usize]);
                } else {
                    println!("  {}", self.options[i as usize]);
                }
            }

            // Handle user input
            if event::poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    match key_event.code {
                        KeyCode::Up => {
                            if current_option > 0 {
                                current_option -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if current_option < num_options - 1 {
                                current_option += 1;
                            }
                        }
                        KeyCode::Enter => {
                            println!("Selected option: {}", self.options[current_option as usize]);
                            break;
                        }
                        _ => {}
                    }
                }

                // Flush the input buffer
                while event::poll(std::time::Duration::from_millis(0)).unwrap() {
                    if let Event::Key(_) = event::read().unwrap() {
                        // Discard the key event
                    }
                }
            }

            handle.flush().unwrap();
        }
    }
}

fn draw_title_screen() {
    clear_console();
    println!();
    println!("   ###### ************************** * *  ");
    println!("   ######                           *     ");
    println!("   ##     #### #### #### #### ####     *  ");
    println!("   ####   #    #    ## # ## # #           ");
    println!("   ##     #### #    #### #### ###    *  * ");
    println!("   ######    # #    ## # ##   #           ");
    println!("   ###### #### #### ## # ##   ####     *  ");
    println!("                                    *     ");
    println!("   ############  =======================  ");
    println!("   #          #   ######                  ");
    println!("   #  @       ?   ##   # #### #### ##   # ");
    println!("   #          #   #####  #  # #  # ### ## ");
    println!("   #     $    #   ##   # #  # #  # ## # # ");
    println!("   #          #   ##   # #  # #  # ##   # ");
    println!("   ############   ##   # #### #### ##   # ");
    println!();
    println!();
    println!();
    println!();
}