use crossterm::event::{self, KeyEvent, KeyCode};
use crate::position::handle_key;

// massive help on keyboard events from this guide
// https://medium.com/@otukof/build-your-text-editor-with-rust-part-2-74e03daef237
// and also some help from this guide
// https://stackoverflow.com/a/60130920/12706133
// but they were both outdated

// ty std::Option <3
// https://doc.rust-lang.org/rust-by-example/std/option.html
pub fn match_keyboard_event(event: KeyEvent) -> Option<bool> {
    match event {
        // documentation on keyEvents
        // https://docs.rs/crossterm/0.27.0/crossterm/event/struct.KeyEvent.html
        // controlling one
        KeyEvent {
            code: KeyCode::Char('w'),
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { handle_key('w', ""); None },
        KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { handle_key('a', ""); None },
        KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { handle_key('s', ""); None },
        KeyEvent {
            code: KeyCode::Char('d'),
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { handle_key('d', ""); None },
        // controlling the other
        KeyEvent {
            code: KeyCode::Left,
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { handle_key(' ', "Left"); None },
        KeyEvent {
            code: KeyCode::Right,
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { handle_key(' ', "Right"); None },
        KeyEvent {
            code: KeyCode::Up,
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { handle_key(' ', "Up"); None },
        KeyEvent {
            code: KeyCode::Down,
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { handle_key(' ', "Down"); None },
        // switch inputs
        KeyEvent {
            code: KeyCode::Tab,
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { handle_key(' ', "Tab"); None },
        KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => { handle_key(' ', "Space"); None },
        // break out from program
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: event::KeyModifiers::CONTROL,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => Some(true),
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: event::KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE
        } => Some(true),
        _ => None,
    }
}