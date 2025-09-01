pub struct App {
    pub current_screen: CurrentScreen,
}
pub enum CurrentScreen {
    Menu,
    Game,
    Over,
}
impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Menu,
        }
    }

    pub fn string(&self) -> String {
        match self.current_screen {
            CurrentScreen::Menu => "Menu",
            _ => "TODO!",
        }
        .to_owned()
    }
}
