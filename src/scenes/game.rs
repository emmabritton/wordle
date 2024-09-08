use crate::engine::{EngineState, WordleEngine};
use crate::scenes::keys_to_input;
use crate::ui::button_bar::{ButtonBar, ButtonDef, BAR_HEIGHT};
use crate::ui::keyboard::{Key, Keyboard};
use crate::ui::theme::colors;
use crate::ui::wordle_renderer::render_field;
use crate::{Input, SceneName, SceneResult, HEIGHT, WIDTH};
use pixels_graphics_lib::prelude::PixelFont::Standard6x7;
use pixels_graphics_lib::prelude::*;

pub struct GameScene {
    engine: WordleEngine,
    keyboard: Keyboard,
    button_bar: ButtonBar,
    end_button_bar: ButtonBar,
    input_timer: Timer,
    show_error: bool,
}

impl GameScene {
    pub fn new(word_size: usize) -> Box<Self> {
        let keyboard_pos = coord!(WIDTH / 2, HEIGHT)
            - (Keyboard::size().0 / 2, Keyboard::size().1)
            - (0_usize, BAR_HEIGHT);
        let engine = WordleEngine::new(word_size);
        println!("{}", engine.word);
        Box::new(GameScene {
            engine,
            show_error: false,
            keyboard: Keyboard::new(keyboard_pos),
            button_bar: ButtonBar::new(
                coord!(0, HEIGHT - BAR_HEIGHT),
                WIDTH,
                &[
                    ("CURSOR", ButtonDef::Cursor),
                    ("SELECT", ButtonDef::Select),
                    ("CLOSE", ButtonDef::Escape),
                ],
            ),
            end_button_bar: ButtonBar::new(
                coord!(0, HEIGHT - BAR_HEIGHT),
                WIDTH,
                &[("CLOSE", ButtonDef::Escape)],
            ),
            input_timer: Timer::new_once(0.3),
        })
    }
}

impl GameScene {
    fn submit(&mut self) {
        if let Ok(result) = self.engine.submit() {
            if let Some((matches, mismatches, no_matches)) = result {
                for c in matches {
                    self.keyboard.add_match(c);
                }
                for c in mismatches {
                    self.keyboard.add_mismatch(c);
                }
                for c in no_matches {
                    self.keyboard.add_no_match(c);
                }
            }
        } else {
            self.show_error = true;
        }
    }
}

impl Scene<SceneResult, SceneName> for GameScene {
    fn render(
        &self,
        graphics: &mut Graphics,
        _: &MouseData,
        _: &FxHashSet<KeyCode>,
        controller: &GameController,
    ) {
        graphics.clear(colors::BACKGROUND);
        self.keyboard.render(graphics);
        render_field(graphics, coord!(0, 0), &self.engine);

        if self.show_error {
            graphics.draw_text(
                "Not a word",
                TextPos::px(coord!(WIDTH / 2, HEIGHT - Keyboard::size().1 - 26)),
                (colors::ERROR, Standard6x7, Positioning::Center),
            );
        }

        match self.engine.state {
            EngineState::Found => {
                draw_end_game(
                    graphics,
                    EndGame::Win(self.engine.guesses.len(), self.engine.max_guess_count),
                );
                self.end_button_bar
                    .render(graphics, controller.get_controller_type());
            }
            EngineState::OutOfGuesses => {
                draw_end_game(graphics, EndGame::Lose);
                self.end_button_bar
                    .render(graphics, controller.get_controller_type());
            }
            EngineState::Guessing => self
                .button_bar
                .render(graphics, controller.get_controller_type()),
        }
    }

    fn on_mouse_click(
        &mut self,
        down_at: Coord,
        mouse: &MouseData,
        mouse_button: MouseButton,
        _: &FxHashSet<KeyCode>,
    ) {
        if mouse_button == MouseButton::Left {
            self.show_error = false;
            if let Some(key) = self.keyboard.mouse_click(down_at, mouse.xy) {
                match key {
                    Key::Letter(chr) => self.engine.add_letter(chr),
                    Key::Enter => self.submit(),
                    Key::Backspace => self.engine.backspace(),
                }
            }
        }
    }

    fn update(
        &mut self,
        timing: &Timing,
        mouse: &MouseData,
        held_keys: &FxHashSet<KeyCode>,
        controller: &GameController,
    ) -> SceneUpdateResult<SceneResult, SceneName> {
        if self.input_timer.update(timing) {
            let input = keys_to_input(held_keys, controller);
            if let Some(input) = input {
                self.show_error = false;
                self.input_timer.reset();
                if let Some(key) = self.keyboard.key_press(input) {
                    match key {
                        Key::Letter(chr) => self.engine.add_letter(chr),
                        Key::Enter => self.submit(),
                        Key::Backspace => self.engine.backspace(),
                    }
                }
            }
            if input == Some(Input::Escape) {
                return SceneUpdateResult::Pop(None);
            }
        }
        self.keyboard.mouse_move(mouse.xy);
        SceneUpdateResult::Nothing
    }
}

enum EndGame {
    //number of guesses made, max guesses
    Win(usize, usize),
    Lose,
}

impl EndGame {
    pub fn back(&self) -> Color {
        match self {
            EndGame::Win(_, _) => colors::WIN_BACK,
            EndGame::Lose => colors::LOSE_BACK,
        }
    }

    pub fn banner(&self) -> Color {
        match self {
            EndGame::Win(_, _) => colors::WIN_BANNER,
            EndGame::Lose => colors::LOSE_BANNER,
        }
    }

    pub fn text(&self) -> Color {
        match self {
            EndGame::Win(_, _) => colors::WIN_TEXT,
            EndGame::Lose => colors::LOSE_TEXT,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            EndGame::Win(_, _) => "Congratulations!",
            EndGame::Lose => "Out of guesses!",
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            EndGame::Win(count, max) => {
                let last = max - 1;
                if *count == last {
                    return "Just in time";
                }
                match *count {
                    1 => "Incredible",
                    2 => "Fantastic",
                    3 => "Great job",
                    _ => "Good job",
                }
            }
            EndGame::Lose => "Better luck next time",
        }
    }
}

fn draw_end_game(graphics: &mut Graphics, end_game: EndGame) {
    let banner_edge_height = 10;
    let banner_back = Rect::new_with_size(
        coord!(0.0, HEIGHT as f32 * 0.40),
        WIDTH,
        ((HEIGHT as f32) * 0.2) as usize,
    );
    let banner_top = Rect::new_with_size(
        banner_back.top_left() - (0, banner_edge_height),
        WIDTH,
        banner_edge_height,
    );
    let banner_bottom = Rect::new_with_size(banner_back.bottom_left(), WIDTH, banner_edge_height);
    let title_pos = banner_back.center() - (0, 5);
    let message_pos = banner_back.center() + (0, 15);

    graphics.clear_aware(Color::new(0, 0, 0, 100));

    graphics.draw_rect(banner_back, fill(end_game.back()));
    graphics.draw_rect(banner_top, fill(end_game.banner()));
    graphics.draw_rect(banner_bottom, fill(end_game.banner()));

    graphics.draw_text(
        end_game.title(),
        TextPos::px(title_pos),
        (
            end_game.text(),
            PixelFont::Standard8x10,
            Positioning::Center,
        ),
    );
    graphics.draw_text(
        end_game.message(),
        TextPos::px(message_pos),
        (end_game.text(), PixelFont::Standard6x7, Positioning::Center),
    );
}
