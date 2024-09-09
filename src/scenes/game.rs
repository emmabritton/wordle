use crate::engine::{EngineState, SubmittedGuessInfo, WordleEngine};
use crate::scenes::keys_to_input;
use crate::ui::button_bar::{ButtonBar, ButtonDef, BAR_HEIGHT};
use crate::ui::keyboard::{Key, Keyboard};
use crate::ui::theme::colors;
use crate::ui::wordle_renderer::{render_field, render_guess_field};
use crate::word_list::word_count_for_size;
use crate::{Input, SceneName, SceneResult, Settings, HEIGHT, WIDTH};
use pixels_graphics_lib::prelude::*;

const ANIM_UPDATE_RATE: f64 = 0.05;
const ANIM_GUESS_STEP: f64 = ANIM_UPDATE_RATE / 5.0;
const ANIM_ENDGAME_STEP: f64 = ANIM_UPDATE_RATE / 1.3;

#[derive(Debug)]
enum GameState {
    Input,
    AnimGuess(SubmittedGuessInfo),
    AnimEndGame,
    GameOver,
}

pub struct GameScene {
    engine: WordleEngine,
    keyboard: Keyboard,
    button_bar: ButtonBar,
    end_button_bar: ButtonBar,
    input_timer: Timer,
    show_error: bool,
    anim_timer: Timer,
    state: GameState,
    anim_perc: f64,
    #[allow(unused)] //needed to play sound
    audio_engine: Option<AudioEngine>,
    win_sound: Option<SoundEffect>,
}

impl GameScene {
    pub fn new(word_size: usize, mut settings: AppPrefs<Settings>) -> Box<Self> {
        let (audio_engine, sound) = if let Ok(engine) = AudioEngine::new() {
            if let Ok(win_sound) =
                engine.load_from_bytes(include_bytes!("../../assets/win.wav"), 1.75)
            {
                (Some(engine), Some(win_sound))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };
        let keyboard_pos = coord!(WIDTH / 2, HEIGHT)
            - (Keyboard::size().0 / 2, Keyboard::size().1)
            - (0_usize, BAR_HEIGHT);
        if settings
            .data
            .word_idx
            .get(&word_size)
            .copied()
            .unwrap_or_default()
            == word_count_for_size(word_size)
        {
            settings.data.word_idx.insert(word_size, 0);
        }
        let engine = WordleEngine::new(
            word_size,
            settings
                .data
                .word_idx
                .get(&word_size)
                .copied()
                .unwrap_or_default(),
        );
        settings
            .data
            .word_idx
            .entry(word_size)
            .and_modify(|v| *v += 1)
            .or_insert(1);
        settings.save();
        Box::new(GameScene {
            engine,
            anim_perc: 0.0,
            audio_engine,
            state: GameState::Input,
            show_error: false,
            keyboard: Keyboard::new(keyboard_pos),
            anim_timer: Timer::new(ANIM_UPDATE_RATE),
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
            win_sound: sound,
        })
    }
}

impl GameScene {
    fn submit(&mut self) {
        if let Ok(result) = self.engine.submit() {
            if let Some(info) = result {
                self.anim_perc = 0.0;
                self.state = GameState::AnimGuess(info);
            }
        } else {
            self.show_error = true;
        }
    }

    fn update_keyboard(&mut self) {
        if let GameState::AnimGuess(info) = &self.state {
            for c in &info.matches {
                self.keyboard.add_match(*c);
            }
            for c in &info.mismatches {
                self.keyboard.add_mismatch(*c);
            }
            for c in &info.no_matches {
                self.keyboard.add_no_match(*c);
            }
            if self.engine.state == EngineState::Guessing {
                self.state = GameState::Input;
            } else {
                self.anim_perc = 0.0;
                self.state = GameState::AnimEndGame;
                if self.engine.state == EngineState::Found {
                    if let Some(sound) = &mut self.win_sound {
                        sound.play();
                    }
                }
            }
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
        if let GameState::AnimGuess(info) = &self.state {
            render_guess_field(
                graphics,
                coord!(0, 0),
                &self.engine,
                info,
                self.anim_perc.clamp(0.0, 1.0),
            );
        } else {
            render_field(graphics, coord!(0, 0), &self.engine);
        }

        if self.show_error {
            graphics.draw_text(
                "Unknown word",
                TextPos::px(coord!(WIDTH / 2, HEIGHT - Keyboard::size().1 - 26)),
                (colors::ERROR, PixelFont::Standard6x7, Positioning::Center),
            );
        }

        match self.engine.state {
            EngineState::Found => {
                if matches!(self.state, GameState::GameOver | GameState::AnimEndGame) {
                    draw_end_game(
                        graphics,
                        self.anim_perc.clamp(0.0, 1.0),
                        EndGame::Win(self.engine.guesses.len(), self.engine.max_guess_count),
                    );
                }
                self.end_button_bar
                    .render(graphics, controller.get_controller_type());
            }
            EngineState::OutOfGuesses => {
                if matches!(self.state, GameState::GameOver | GameState::AnimEndGame) {
                    draw_end_game(graphics, self.anim_perc.clamp(0.0, 1.0), EndGame::Lose);
                }
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
        if matches!(self.state, GameState::Input) && mouse_button == MouseButton::Left {
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
        if let Some(sound) = &mut self.win_sound {
            sound.update(timing);
        }
        if self.input_timer.update(timing) {
            let input = keys_to_input(held_keys, controller);
            if matches!(self.state, GameState::Input) {
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
            }
            if input == Some(Input::Escape) {
                return SceneUpdateResult::Pop(None);
            }
        }
        if self.anim_timer.update(timing) {
            match self.state {
                GameState::Input => {}
                GameState::AnimGuess(_) => {
                    self.anim_perc += ANIM_GUESS_STEP;
                    if self.anim_perc >= 1.0 {
                        self.update_keyboard();
                    }
                }
                GameState::AnimEndGame => {
                    self.anim_perc += ANIM_ENDGAME_STEP;
                    if self.anim_perc >= 1.0 {
                        self.state = GameState::GameOver
                    }
                }
                GameState::GameOver => {}
            }
        }

        self.keyboard.mouse_move(mouse.xy);
        SceneUpdateResult::Nothing
    }
}

#[derive(Debug, Clone)]
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

fn draw_end_game(graphics: &mut Graphics, perc: f64, end_game: EndGame) {
    let anim_offset = coord!(WIDTH.lerp(0, perc as f32), 0);
    let text_offset = (WIDTH * 2).lerp(0, perc as f32);
    let banner_edge_height = 10;
    let banner_back = Rect::new_with_size(
        coord!(0.0, HEIGHT as f32 * 0.40) + anim_offset,
        WIDTH,
        ((HEIGHT as f32) * 0.2) as usize,
    );
    let banner_top = Rect::new_with_size(
        banner_back.top_left() - (0, banner_edge_height),
        WIDTH,
        banner_edge_height,
    );
    let banner_bottom = Rect::new_with_size(banner_back.bottom_left(), WIDTH, banner_edge_height);
    let title_pos = banner_back.center() - (text_offset, 5);
    let message_pos = banner_back.center() + (text_offset, 15);

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

    let alpha = (inv_flerp(0.0, 1.0, perc as f32) * 100.0) as u8;
    graphics.draw_rect(
        Rect::new_with_size(coord!(0, HEIGHT - BAR_HEIGHT + 2), WIDTH, BAR_HEIGHT),
        fill(Color::new(0, 0, 0, alpha)),
    );
}
