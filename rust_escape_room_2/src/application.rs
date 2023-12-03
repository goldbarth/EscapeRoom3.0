use crate::menu::Menu;

enum GameState {
    MainMenu,
    Game,
    Tutorial,
    Exit,
}

enum MenuType {
    MainMenu,
    Tutorial,
    Outro,
    Exit,
}

impl GameState {
    fn new() -> GameState {
        GameState::MainMenu
    }
}

pub struct Application {
    game_state: GameState,
}

impl Application {
    pub fn new() -> Self {
        Application {
            game_state: GameState::MainMenu,
        }
    }

    pub fn run(&mut self) {
        self.update();
    }

    fn update(&mut self) {
        loop {
            match self.game_state {
                GameState::MainMenu => self.initialize_state(MenuType::MainMenu),
                GameState::Game => self.initialize_state(MenuType::MainMenu),
                GameState::Tutorial => self.initialize_state(MenuType::Tutorial),
                GameState::Exit => self.initialize_state(MenuType::Exit),
            }
        }
    }

    fn initialize_state(&self, menu_type: MenuType) {
        let menu = Menu::new();
        match menu_type {
            MenuType::MainMenu => menu.initialize_main_menu(),
            MenuType::Tutorial => menu.initialize_tutorial(),
            MenuType::Outro => menu.initialize_outro(),
            MenuType::Exit => menu.initialize_exit(),
        }
    }

    fn start_outro(&self) {
        self.initialize_state(MenuType::Outro);
    }

    fn set_state(&mut self, state: GameState) {
        self.game_state = state;
    }
}