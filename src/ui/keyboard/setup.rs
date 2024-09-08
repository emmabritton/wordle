use crate::ui::keyboard::{Input, Key};
use pixels_graphics_lib::prelude::*;
use std::collections::HashMap;
use std::sync::OnceLock;

pub const SPACING: usize = 4;
pub const KEY_SIZE_LETTER: (usize, usize) = (11, 15);
pub const KEY_SIZE_ACTION: (usize, usize) = (18, 15);

static KEYS: OnceLock<Vec<Vec<Key>>> = OnceLock::new();
static CHECK: OnceLock<IndexedImage> = OnceLock::new();
static BACKSPACE: OnceLock<IndexedImage> = OnceLock::new();
static KEY_POS: OnceLock<HashMap<Key, Coord>> = OnceLock::new();

const fn calc_key_pos(key: Key) -> (usize, usize) {
    let row1_y = 0;
    let row2_y = SPACING + KEY_SIZE_LETTER.1;
    let row3_y = (SPACING + KEY_SIZE_LETTER.1) * 2;
    let row2_x = 7; // ceil(KEY_SIZE_ACTION.0 / 2) + 1?
    let row3_x = KEY_SIZE_ACTION.0 + SPACING;
    const fn calc_x(count: usize) -> usize {
        count * (SPACING + KEY_SIZE_LETTER.0)
    }
    match key {
        Key::Letter(chr) => match chr {
            'Q' => (0, row1_y),
            'A' => (row2_x, row2_y),
            'Z' => (row3_x, row3_y),
            'W' => (calc_x(1), row1_y),
            'S' => (calc_x(1) + row2_x, row2_y),
            'X' => (calc_x(1) + row3_x, row3_y),
            'E' => (calc_x(2), row1_y),
            'D' => (calc_x(2) + row2_x, row2_y),
            'C' => (calc_x(2) + row3_x, row3_y),
            'R' => (calc_x(3), row1_y),
            'F' => (calc_x(3) + row2_x, row2_y),
            'V' => (calc_x(3) + row3_x, row3_y),
            'T' => (calc_x(4), row1_y),
            'G' => (calc_x(4) + row2_x, row2_y),
            'B' => (calc_x(4) + row3_x, row3_y),
            'Y' => (calc_x(5), row1_y),
            'H' => (calc_x(5) + row2_x, row2_y),
            'N' => (calc_x(5) + row3_x, row3_y),
            'U' => (calc_x(6), row1_y),
            'J' => (calc_x(6) + row2_x, row2_y),
            'M' => (calc_x(6) + row3_x, row3_y),
            'I' => (calc_x(7), row1_y),
            'K' => (calc_x(7) + row2_x, row2_y),
            'O' => (calc_x(8), row1_y),
            'L' => (calc_x(8) + row2_x, row2_y),
            'P' => (calc_x(9), row1_y),
            _ => (999, 999), // panic!("Invalid chr"),
        },
        Key::Enter => (0, row3_y),
        Key::Backspace => (calc_x(7) + SPACING + KEY_SIZE_ACTION.0, row3_y),
    }
}

pub fn backspace() -> &'static IndexedImage {
    BACKSPACE.get_or_init(|| {
        IndexedImage::from_file_contents(include_bytes!("../../../assets/icons/backspace.ici"))
            .unwrap()
            .0
    })
}

pub fn check() -> &'static IndexedImage {
    CHECK.get_or_init(|| {
        IndexedImage::from_file_contents(include_bytes!("../../../assets/icons/check.ici"))
            .unwrap()
            .0
    })
}

pub fn keys() -> &'static Vec<Vec<Key>> {
    KEYS.get_or_init(|| {
        let mut output: Vec<Vec<Key>> = vec![
            "QWERTYUIOP".chars().map(Key::Letter).collect::<Vec<Key>>(),
            "ASDFGHJKL".chars().map(Key::Letter).collect::<Vec<Key>>(),
            "ZXCVBNM".chars().map(Key::Letter).collect::<Vec<Key>>(),
        ];
        output[2].insert(0, Key::Enter);
        output[2].push(Key::Backspace);
        output
    })
}

pub fn key_pos() -> &'static HashMap<Key, Coord> {
    KEY_POS.get_or_init(|| {
        let mut output: HashMap<Key, Coord> = HashMap::new();

        for row in keys() {
            for key in row {
                output.insert(*key, coord!(calc_key_pos(*key)));
            }
        }

        output
    })
}

fn l(chr: char) -> Key {
    Key::Letter(chr)
}

pub fn move_cursor(input: Input, cursor: Key) -> Option<Key> {
    //outcomes (up, right, down, left)
    fn handle_input(input: Input, outcomes: [Key; 4]) -> Key {
        match input {
            Input::Up => outcomes[0],
            Input::Down => outcomes[2],
            Input::Left => outcomes[3],
            Input::Right => outcomes[1],
            _ => panic!("invalid input: {input:?}"),
        }
    }
    if input == Input::Action || input == Input::Escape {
        return None;
    }
    let new_key = match cursor {
        Key::Letter(chr) => match chr {
            'Q' => handle_input(input, [Key::Enter, l('W'), l('A'), l('P')]),
            'W' => handle_input(input, [Key::Enter, l('E'), l('S'), l('Q')]),
            'E' => handle_input(input, [l('X'), l('R'), l('D'), l('W')]),
            'R' => handle_input(input, [l('C'), l('T'), l('F'), l('E')]),
            'T' => handle_input(input, [l('V'), l('Y'), l('G'), l('R')]),
            'Y' => handle_input(input, [l('B'), l('U'), l('H'), l('T')]),
            'U' => handle_input(input, [l('N'), l('I'), l('J'), l('Y')]),
            'I' => handle_input(input, [l('M'), l('O'), l('K'), l('U')]),
            'O' => handle_input(input, [Key::Backspace, l('P'), l('L'), l('I')]),
            'P' => handle_input(input, [Key::Backspace, l('Q'), l('L'), l('O')]),

            'A' => handle_input(input, [l('Q'), l('S'), Key::Enter, l('L')]),
            'S' => handle_input(input, [l('W'), l('D'), l('Z'), l('A')]),
            'D' => handle_input(input, [l('E'), l('F'), l('X'), l('S')]),
            'F' => handle_input(input, [l('R'), l('G'), l('C'), l('D')]),
            'G' => handle_input(input, [l('T'), l('H'), l('V'), l('F')]),
            'H' => handle_input(input, [l('Y'), l('J'), l('B'), l('G')]),
            'J' => handle_input(input, [l('U'), l('K'), l('N'), l('H')]),
            'K' => handle_input(input, [l('I'), l('L'), l('M'), l('J')]),
            'L' => handle_input(input, [l('O'), l('A'), Key::Backspace, l('K')]),

            'Z' => handle_input(input, [l('S'), l('X'), l('W'), Key::Enter]),
            'X' => handle_input(input, [l('D'), l('C'), l('E'), l('Z')]),
            'C' => handle_input(input, [l('F'), l('V'), l('R'), l('X')]),
            'V' => handle_input(input, [l('G'), l('B'), l('T'), l('C')]),
            'B' => handle_input(input, [l('H'), l('N'), l('Y'), l('V')]),
            'N' => handle_input(input, [l('J'), l('M'), l('U'), l('B')]),
            'M' => handle_input(input, [l('K'), Key::Backspace, l('I'), l('N')]),
            _ => panic!("Cursor on invalid letter {chr}"),
        },
        Key::Enter => handle_input(input, [l('A'), l('Z'), l('Q'), Key::Backspace]),
        Key::Backspace => handle_input(input, [l('L'), Key::Enter, l('P'), l('M')]),
    };
    Some(new_key)
}
