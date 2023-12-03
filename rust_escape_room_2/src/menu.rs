use std::io::{self, Write};
use std::process::exit;
use crate::console::clear_console;
use crate::console::set_cursor_position;
use crossterm::{
    event::{KeyCode, KeyEvent, Event, KeyModifiers, read},
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

        let current_line = self.initial_line + 18;
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
        let num_options = self.options.len();

        let mut current_option = 0;
        //let mut current_line = self.initial_line;
        let mut has_selected = false;

        while !has_selected {
            let start_position = 0;
            // Clear only the part of the console where the options are displayed
            for _ in 0..num_options + 1 {
                let char_count = 80;
                let space_char = ' ';
                set_cursor_position(start_position, current_line);
                println!("{:1$}", space_char, char_count);
                current_line += 1;
            }

            // Draw the menu options witch a > in front of the selected option and highlight the selected option
            // TODO: Find out why the up and down arrow keys are not working
            for i in 0..num_options {
                set_cursor_position(start_position, current_line);

                if i == current_option {

                    println!("> {}", self.options[i]);
                } else {
                    println!("  {}", self.options[i]);
                }

                current_line += 1;
            }

            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read user input");

            if let Ok(event) = read() {
                match event {
                    Event::Key(KeyEvent {
                                   code,
                                   modifiers,
                                   kind: _,
                                   state: _,
                               }) => {
                        if modifiers == KeyModifiers::NONE {
                            match code {
                                KeyCode::Enter => {
                                    has_selected = true;
                                    match current_option {
                                        0 => println!("Start Game"),
                                        1 => println!("Exit"),
                                        2 => println!("Tutorial"),
                                        _ => println!("Invalid option"),
                                    }
                                }
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
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }

            //current_line = self.initial_line;
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
}