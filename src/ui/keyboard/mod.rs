mod setup;

use crate::ui::keyboard::setup::*;
use crate::ui::theme::colors;
use crate::Input;
use pixels_graphics_lib::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Key {
    Letter(char),
    Enter,
    Backspace,
}

impl Key {
    pub fn size(&self) -> (usize, usize) {
        match self {
            Key::Letter(_) => KEY_SIZE_LETTER,
            _ => KEY_SIZE_ACTION,
        }
    }
}

pub struct Keyboard {
    pos: Coord,
    cursor: Key,
    //mouse cursor
    last_pos: Coord,
    matched: Vec<char>,
    mismatched: Vec<char>,
    no_matches: Vec<char>,
}

impl Keyboard {
    pub fn size() -> (usize, usize) {
        (
            (KEY_SIZE_LETTER.0 * 10) + (SPACING * 9),
            (KEY_SIZE_LETTER.1 * 3) + (SPACING * 2),
        )
    }
}

impl Keyboard {
    pub fn new(pos: Coord) -> Self {
        Keyboard {
            pos,
            cursor: Key::Letter('G'),
            last_pos: coord!(-1, -1),
            matched: vec![],
            mismatched: vec![],
            no_matches: vec![],
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum KeyState {
    Default,
    Mismatch,
    Match,
    NoMatch,
}

impl Keyboard {
    fn is_known(&self, chr: char) -> bool {
        self.matched.contains(&chr)
            || self.no_matches.contains(&chr)
            || self.mismatched.contains(&chr)
    }

    pub fn add_mismatch(&mut self, chr: char) {
        if !self.is_known(chr) {
            self.mismatched.push(chr);
        }
    }

    pub fn add_match(&mut self, chr: char) {
        if !(self.matched.contains(&chr) || self.no_matches.contains(&chr)) {
            self.matched.push(chr);
            if let Some(idx) = self.mismatched.iter().position(|c| c == &chr) {
                self.mismatched.remove(idx);
            }
        }
    }

    pub fn add_no_match(&mut self, chr: char) {
        if !self.is_known(chr) {
            self.no_matches.push(chr);
        }
    }

    pub fn render(&self, graphics: &mut Graphics) {
        let start = self.pos;
        let key_pos = key_pos();

        for row in keys() {
            for key in row {
                let pos = start + key_pos[key];
                draw_key(
                    graphics,
                    *key,
                    pos,
                    self.cursor == *key,
                    self.state_for(*key),
                );
            }
        }
    }

    pub fn mouse_click(&mut self, down_at: Coord, up_at: Coord) -> Option<Key> {
        let down_at = down_at - self.pos;
        let up_at = up_at - self.pos;
        for (key, pos) in key_pos() {
            let size = key.size();
            let area = Rect::new_with_size(pos, size.0, size.1);
            if area.contains(down_at) && area.contains(up_at) {
                return Some(*key);
            }
        }
        None
    }

    pub fn mouse_move(&mut self, xy: Coord) {
        if xy == self.last_pos {
            return;
        }
        self.last_pos = xy;
        let xy = xy - self.pos;
        for (key, pos) in key_pos() {
            let size = key.size();
            let area = Rect::new_with_size(pos, size.0, size.1);
            if area.contains(xy) {
                self.cursor = *key;
            }
        }
    }

    pub fn key_press(&mut self, input: Input) -> Option<Key> {
        match input {
            Input::Action => return Some(self.cursor),
            _ => {
                if let Some(key) = move_cursor(input, self.cursor) {
                    self.cursor = key;
                }
            }
        }
        None
    }

    fn state_for(&self, key: Key) -> KeyState {
        match key {
            Key::Letter(c) => {
                if self.matched.contains(&c) {
                    KeyState::Match
                } else if self.mismatched.contains(&c) {
                    KeyState::Mismatch
                } else if self.no_matches.contains(&c) {
                    KeyState::NoMatch
                } else {
                    KeyState::Default
                }
            }
            Key::Enter => KeyState::Default,
            Key::Backspace => KeyState::Default,
        }
    }
}

fn draw_key(graphics: &mut Graphics, key: Key, pos: Coord, highlighted: bool, state: KeyState) {
    let size = key.size();
    let rect = Rect::new_with_size(pos, size.0, size.1);
    let (back_clr, fore_clr) = match state {
        KeyState::Default => (colors::KEYBOARD_BACK, colors::KEYBOARD_LETTER),
        KeyState::Mismatch => (colors::SLOT_POS_WRONG_BACK, colors::SLOT_POS_WRONG_FORE),
        KeyState::Match => (colors::SLOT_POS_RIGHT_BACK, colors::SLOT_POS_RIGHT_FORE),
        KeyState::NoMatch => (colors::SLOT_NO_MATCH_BACK, colors::SLOT_NO_MATCH_FORE),
    };
    graphics.draw_rect(rect.clone(), fill(back_clr));
    match key {
        Key::Letter(chr) => graphics.draw_text(
            &chr.to_string(),
            TextPos::px(rect.center() + (1, 1)),
            (fore_clr, PixelFont::Standard4x5, Positioning::Center),
        ),
        Key::Enter => graphics.draw_indexed_image(rect.center() - (5, 3), check()),
        Key::Backspace => graphics.draw_indexed_image(rect.center() - (8, 4), backspace()),
    }
    if highlighted {
        graphics.draw_rect(rect, stroke(colors::KEYBOARD_HIGHLIGHT))
    }
}
