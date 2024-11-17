use crate::Context;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, ModifierKeyCode};
use enumset::EnumSetType;

impl Context {
    pub fn handle_keyboard_event(&mut self, event: KeyEvent) {
        let key = Key::from(&event.code);
        match event.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => {
                self.current_state.keys_down.insert(key);
            }
            KeyEventKind::Release => {
                self.current_state.keys_down.remove(key);
            }
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.current_state.keys_down.contains(key)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.current_state.keys_down.contains(key) && !self.previous_state.keys_down.contains(key)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        !self.current_state.keys_down.contains(key) && self.previous_state.keys_down.contains(key)
    }
}

#[derive(Debug, EnumSetType)]
#[rustfmt::skip]
pub enum Key {
    Unknown,

    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    LeftShift, RightShift,
    LeftControl, RightControl,
    Space,
    // Escape,

    // Keys that I don't feel like handling rn
    // Backspace,
    // Enter,
    // Left, Right, Up, Down,
    // Home, End,
    // PageUp, PageDown,
    // Tab,
    // Delete,
    // Insert,
    // F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    // Keypad0, Keypad1, Keypad2, Keypad3, Keypad4, Keypad5, Keypad6, Keypad7, Keypad8, Keypad9,
    // CapsLock,
    // ScrollLock,
    // NumLock,
    // PrintScreen,
    // LeftAlt, RightAlt,
    // LeftSuper, RightSuper,
    // Apostrophe,
    // Comma,
    // Minus,
    // Period,
    // Slash, Backslash,
    // Semicolon,
    // Equals,
    // LeftBracket, RightBracket,
    // GraveAccent,

    // WTF ARE EVEN THOSE
    // BackTab,
    // Pause,
    // Menu,
    // KeypadBegin, MediaPlay, MediaPause,
    // MediaPlayPause, MediaReverse, MediaStop,
    // MediaFastForward, MediaRewind, MediaTrackNext,
    // MediaTrackPrevious, MediaRecord, MediaLowerVolume,
    // MediaRaiseVolume, MediaMuteVolume, 
    // LeftHyper, RightHyper,
    // LeftMeta, RightMeta,
    // IsoLevel3Shift, IsoLevel5Shift,
    // World1, World2,
}

impl From<&KeyCode> for Key {
    fn from(key: &KeyCode) -> Self {
        use KeyCode::*;
        use ModifierKeyCode::*;
        match key {
            Char(c) => match c {
                'a' => Key::A,
                'b' => Key::B,
                'c' => Key::C,
                'd' => Key::D,
                'e' => Key::E,
                'f' => Key::F,
                'g' => Key::G,
                'h' => Key::H,
                'i' => Key::I,
                'j' => Key::J,
                'k' => Key::K,
                'l' => Key::L,
                'm' => Key::M,
                'n' => Key::N,
                'o' => Key::O,
                'p' => Key::P,
                'q' => Key::Q,
                'r' => Key::R,
                's' => Key::S,
                't' => Key::T,
                'u' => Key::U,
                'v' => Key::V,
                'w' => Key::W,
                'x' => Key::X,
                'y' => Key::Y,
                'z' => Key::Z,
                ' ' => Key::Space,
                '0' => Key::Num0,
                '1' => Key::Num1,
                '2' => Key::Num2,
                '3' => Key::Num3,
                '4' => Key::Num4,
                '5' => Key::Num5,
                '6' => Key::Num6,
                '7' => Key::Num7,
                '8' => Key::Num8,
                '9' => Key::Num9,
                _ => Key::Unknown,
                // _ => panic!("{:?}", key),
            },
            Modifier(m) => match m {
                LeftShift => Key::LeftShift,
                RightShift => Key::RightShift,
                LeftControl => Key::LeftControl,
                RightControl => Key::RightControl,
                _ => Key::Unknown,
                // _ => panic!("{:?}", key),
            },
            _ => Key::Unknown,
            // _ => panic!("{:?}", key),
        }
    }
}
