use crate::scenes::keys_to_input;
use crate::ui::button_bar::{ButtonBar, ButtonDef, BAR_HEIGHT};
use crate::ui::theme::colors;
use crate::word_list::{FIVE, FOUR, SEVEN, SIX};
use crate::{Input, SceneName, SceneResult, Settings, HEIGHT, WIDTH};
use pixels_graphics_lib::prelude::*;

const BUTTON_START: Coord =
    Coord::new((WIDTH / 2 - 60) as isize, ((HEIGHT as f32) * 0.52) as isize);
const BUTTON_SIZE: (usize, usize) = (120, 20);

const SIZE_BUTTON_SIZE: usize = 20;
const SIZE_BUTTON_START: Coord = Coord::new(
    (WIDTH / 2 - SIZE_BUTTON_SIZE * 2) as isize,
    ((HEIGHT as f32) * 0.3) as isize,
);

const SIZE_REMAINING_POS: Coord =
    Coord::new((WIDTH / 2) as isize, ((HEIGHT as f32) * 0.3) as isize + 32);

pub struct MenuScene {
    result: Option<SceneUpdateResult<SceneResult, SceneName>>,
    size_idx: usize,
    button_idx: usize,
    button_bar: ButtonBar,
    input_timer: Timer,
    size_idxs: [usize; 4],
}

impl MenuScene {
    pub fn new(settings: AppPrefs<Settings>) -> Box<Self> {
        Box::new(MenuScene {
            size_idx: 1,
            button_idx: 0,
            button_bar: ButtonBar::new(
                coord!(0, HEIGHT - BAR_HEIGHT),
                WIDTH,
                &[
                    ("SELECT", ButtonDef::Select),
                    ("WORD SIZE", ButtonDef::Horz),
                    ("BUTTON", ButtonDef::Vert),
                    ("CLOSE", ButtonDef::Escape),
                ],
            ),
            input_timer: Timer::new_once(0.3),
            result: None,
            size_idxs: [
                settings.data.word_idx.get(&4).copied().unwrap_or_default(),
                settings.data.word_idx.get(&5).copied().unwrap_or_default(),
                settings.data.word_idx.get(&6).copied().unwrap_or_default(),
                settings.data.word_idx.get(&7).copied().unwrap_or_default(),
            ],
        })
    }
}

impl MenuScene {
    fn draw_size_buttons(&self, graphics: &mut Graphics) {
        for i in 0..4 {
            let rect = Rect::new_with_size(
                SIZE_BUTTON_START + (i * SIZE_BUTTON_SIZE, 0) - (1, 0),
                SIZE_BUTTON_SIZE,
                SIZE_BUTTON_SIZE,
            );
            graphics.draw_text(
                &format!("{}", i + 4),
                TextPos::px(rect.center() + (1, 1)),
                (
                    colors::MENU_SELECTED,
                    PixelFont::Standard6x7,
                    Positioning::Center,
                ),
            );
            graphics.draw_rect(rect, stroke(colors::MENU_DEFAULT));
        }

        let max = match self.size_idx {
            0 => FOUR.len(),
            1 => FIVE.len(),
            2 => SIX.len(),
            3 => SEVEN.len(),
            _ => panic!("invalid size_idx {}", self.size_idx),
        };
        let count = self.size_idxs[self.size_idx];

        let (text, color) = if max == count {
            ("All done!".to_string(), colors::WIN_BANNER)
        } else {
            (format!("{}/{}", count, max), colors::MENU_DEFAULT)
        };
        graphics.draw_text(
            &text,
            TextPos::px(SIZE_REMAINING_POS),
            (color, PixelFont::Standard6x7, Positioning::Center),
        );

        let rect = Rect::new_with_size(
            SIZE_BUTTON_START + (self.size_idx * SIZE_BUTTON_SIZE, 0) - (1, 0),
            SIZE_BUTTON_SIZE,
            SIZE_BUTTON_SIZE,
        );
        graphics.draw_rect(rect, stroke(colors::MENU_SELECTED));
    }

    fn draw_buttons(&self, graphics: &mut Graphics) {
        let draw_button = |g: &mut Graphics, text: &str, pos: Coord, selected: bool| {
            let rect = Rect::new_with_size(pos, BUTTON_SIZE.0, BUTTON_SIZE.1);
            let back = if selected {
                colors::MENU_SELECTED
            } else {
                colors::MENU_DEFAULT
            };
            g.draw_text(
                text,
                TextPos::px(rect.center() + (1, 1)),
                (
                    colors::MENU_SELECTED,
                    PixelFont::Standard6x7,
                    Positioning::Center,
                ),
            );
            g.draw_rect(rect, stroke(back));
        };

        draw_button(graphics, "PLAY", BUTTON_START, self.button_idx == 0);
        draw_button(
            graphics,
            "EXIT",
            BUTTON_START + (0, 30),
            self.button_idx == 1,
        );
    }
}

impl Scene<SceneResult, SceneName> for MenuScene {
    fn render(
        &self,
        graphics: &mut Graphics,
        _: &MouseData,
        _: &FxHashSet<KeyCode>,
        controller: &GameController,
    ) {
        graphics.clear(colors::BACKGROUND);
        graphics.draw_text(
            "Wordle!",
            TextPos::px(coord!(WIDTH / 2, 40)),
            (
                colors::KEYBOARD_LETTER,
                PixelFont::Standard8x10,
                Positioning::Center,
            ),
        );
        self.button_bar
            .render(graphics, controller.get_controller_type());

        self.draw_size_buttons(graphics);
        self.draw_buttons(graphics);
    }

    fn on_mouse_click(
        &mut self,
        down_at: Coord,
        mouse: &MouseData,
        mouse_button: MouseButton,
        _: &FxHashSet<KeyCode>,
    ) {
        if mouse_button == MouseButton::Left {
            let size_rect =
                Rect::new_with_size(SIZE_BUTTON_START, SIZE_BUTTON_SIZE * 4, SIZE_BUTTON_SIZE);
            if size_rect.contains(down_at) && size_rect.contains(mouse.xy) {
                let down = (down_at - size_rect.top_left()).x.max(0) as usize / SIZE_BUTTON_SIZE;
                let up = (mouse.xy - size_rect.top_left()).x.max(0) as usize / SIZE_BUTTON_SIZE;
                if down == up {
                    self.size_idx = down;
                }
            }
            let play_button = Rect::new_with_size(BUTTON_START, BUTTON_SIZE.0, BUTTON_SIZE.1);
            if play_button.contains(down_at) && play_button.contains(mouse.xy) {
                self.result = Some(SceneUpdateResult::Push(
                    false,
                    SceneName::Game(self.size_idx + 4),
                ));
            }
            let exit_button =
                Rect::new_with_size(BUTTON_START + (0, 30), BUTTON_SIZE.0, BUTTON_SIZE.1);
            if exit_button.contains(down_at) && exit_button.contains(mouse.xy) {
                self.result = Some(SceneUpdateResult::Pop(None));
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
            if let Some(input) = keys_to_input(held_keys, controller) {
                self.input_timer.reset();
                match input {
                    Input::Action => match self.button_idx {
                        0 => {
                            return SceneUpdateResult::Push(
                                false,
                                SceneName::Game(self.size_idx + 4),
                            )
                        }
                        1 => return SceneUpdateResult::Pop(None),
                        _ => {}
                    },
                    Input::Up => {
                        if self.button_idx == 0 {
                            self.button_idx = 1;
                        } else {
                            self.button_idx -= 1;
                        }
                    }
                    Input::Down => {
                        if self.button_idx == 1 {
                            self.button_idx = 0;
                        } else {
                            self.button_idx += 1;
                        }
                    }
                    Input::Left => {
                        if self.size_idx == 0 {
                            self.size_idx = 3;
                        } else {
                            self.size_idx -= 1;
                        }
                    }
                    Input::Right => {
                        if self.size_idx == 3 {
                            self.size_idx = 0;
                        } else {
                            self.size_idx += 1;
                        }
                    }
                    Input::Escape => return SceneUpdateResult::Pop(None),
                }
            }
        }

        let size_rect =
            Rect::new_with_size(SIZE_BUTTON_START, SIZE_BUTTON_SIZE * 4, SIZE_BUTTON_SIZE);
        if size_rect.contains(mouse.xy) {
            self.size_idx =
                (((mouse.xy - SIZE_BUTTON_START).x as usize) / SIZE_BUTTON_SIZE).clamp(0, 3);
        }

        self.result.clone().unwrap_or(SceneUpdateResult::Nothing)
    }

    fn resuming(&mut self, _: Option<SceneResult>) {
        self.result = None;
        self.input_timer.reset();
    }
}
