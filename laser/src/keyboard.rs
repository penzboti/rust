use crossterm::event::{self, KeyEvent, KeyCode};

// massive help on keyboard events from this guide
// https://medium.com/@otukof/build-your-text-editor-with-rust-part-2-74e03daef237
// and also some help from this guide
// https://stackoverflow.com/a/60130920/12706133
// but they were both outdated

// ty std::Option <3
// https://doc.rust-lang.org/rust-by-example/std/option.html
pub fn match_keyboard_event(event: KeyEvent) -> Option<String> {
    let mut key: &str = "";
    match event {
        // documentation on keyEvents
        // https://docs.rs/crossterm/0.27.0/crossterm/event/struct.KeyEvent.html
        // detecting press
        KeyEvent {
            code: KeyCode::Char('w'),
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { key = "w_key"; },
        KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { key = "a_key"; },
        KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { key = "s_key"; },
        KeyEvent {
            code: KeyCode::Char('d'),
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { key = "d_key"; },
        // controlling the other
        KeyEvent {
            code: KeyCode::Left,
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { key = "Left"; },
        KeyEvent {
            code: KeyCode::Right,
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { key = "Right"; },
        KeyEvent {
            code: KeyCode::Up,
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { key = "Up"; },
        KeyEvent {
            code: KeyCode::Down,
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { key = "Down"; },

        // switch inputs
        KeyEvent {
            code: KeyCode::Tab,
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { key = "Tab"; },
        KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { key = "Space"; },
        // break out from program
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: event::KeyModifiers::CONTROL,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => key = "break",
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => key = "break",
        _ => {},
    }
    match key {
        "break" => None,
        _ => Some(key.to_string()),
    }
}