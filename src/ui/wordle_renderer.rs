use crate::engine::{SlotState, WordleEngine};
use crate::ui::theme::colors;
use crate::WIDTH;
use pixels_graphics_lib::prelude::*;

const SPACING: usize = 6;
const FONT: PixelFont = PixelFont::Standard8x10;
const SQUARE_SIZE: (usize, usize) = (FONT.size().0 * 2, FONT.size().1 * 2);
const PADDED_SIZE: (usize, usize) = (FONT.size().0 * 2 + SPACING, FONT.size().1 * 2 + SPACING);
const LETTER_OFFSET: (usize, usize) = (1, 2);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Slot {
    Empty,
    Guess(char),
    NoMatch(char),
    Mismatch(char),
    Match(char),
}

pub fn render_field(graphics: &mut Graphics, offset: Coord, engine: &WordleEngine) {
    let width = (PADDED_SIZE.0) * engine.word_size;

    let offset = offset + ((WIDTH / 2) - (width / 2), SPACING);

    let mut field = vec![];

    convert_guesses(&mut field, engine);
    convert_current_guess(&mut field, engine);
    add_empties(&mut field, engine);

    graphics.with_translate(offset, |g| {
        for (y, row) in field.iter().enumerate() {
            for (x, slot) in row.iter().enumerate() {
                let pos = coord!(x, y) * PADDED_SIZE;
                match slot {
                    Slot::Empty => draw_empty(g, pos),
                    Slot::Guess(c) => draw_guess(g, pos, *c),
                    Slot::Match(_) | Slot::Mismatch(_) | Slot::NoMatch(_) => {
                        draw_answer(g, pos, *slot)
                    }
                }
            }
        }
    });
}

fn draw_answer(graphics: &mut Graphics, pos: Coord, slot: Slot) {
    let (back_color, fore_color, chr) = match slot {
        Slot::Guess(_) | Slot::Empty => panic!("Invalid slot {slot:?} passed"),
        Slot::NoMatch(chr) => (colors::SLOT_NO_MATCH_BACK, colors::SLOT_NO_MATCH_FORE, chr),
        Slot::Mismatch(chr) => (
            colors::SLOT_POS_WRONG_BACK,
            colors::SLOT_POS_WRONG_FORE,
            chr,
        ),
        Slot::Match(chr) => (
            colors::SLOT_POS_RIGHT_BACK,
            colors::SLOT_POS_RIGHT_FORE,
            chr,
        ),
    };
    let rect = Rect::new_with_size(pos, SQUARE_SIZE.0, SQUARE_SIZE.1);
    graphics.draw_rect(rect.clone(), fill(back_color));
    graphics.draw_text(
        &chr.to_string(),
        TextPos::px(rect.center() + LETTER_OFFSET),
        (fore_color, FONT, Positioning::Center),
    );
}

fn draw_guess(graphics: &mut Graphics, pos: Coord, chr: char) {
    let rect = Rect::new_with_size(pos, SQUARE_SIZE.0, SQUARE_SIZE.1);
    graphics.draw_rect(rect.clone(), stroke(colors::SLOT_GUESS_BORDER));
    graphics.draw_text(
        &chr.to_string(),
        TextPos::px(rect.center() + LETTER_OFFSET),
        (colors::SLOT_GUESS_LETTER, FONT, Positioning::Center),
    );
}

fn draw_empty(graphics: &mut Graphics, pos: Coord) {
    let rect = Rect::new_with_size(pos, SQUARE_SIZE.0, SQUARE_SIZE.1);
    graphics.draw_rect(rect, stroke(colors::SLOT_EMPTY_BORDER));
}

fn convert_guesses(field: &mut Vec<Vec<Slot>>, engine: &WordleEngine) {
    for row_idx in 0..engine.max_guess_count {
        if let Some(row) = engine.guesses.get(row_idx) {
            let mut render_row = vec![];
            for slot in row {
                let slot_state = match slot.state {
                    SlotState::Match => Slot::Match(slot.chr),
                    SlotState::WrongPos => Slot::Mismatch(slot.chr),
                    SlotState::NoMatch => Slot::NoMatch(slot.chr),
                };
                render_row.push(slot_state);
            }
            field.push(render_row);
        }
    }
}

fn convert_current_guess(field: &mut Vec<Vec<Slot>>, engine: &WordleEngine) {
    if field.len() < engine.max_guess_count {
        let mut row = vec![];
        for i in 0..engine.word_size {
            row.push(
                engine
                    .current_guess
                    .get(i)
                    .map(|c| Slot::Guess(*c))
                    .unwrap_or(Slot::Empty),
            );
        }
        field.push(row);
    }
}

fn add_empties(field: &mut Vec<Vec<Slot>>, engine: &WordleEngine) {
    for _ in field.len()..engine.max_guess_count {
        field.push(vec![Slot::Empty; engine.word_size]);
    }
}
