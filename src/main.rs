mod engine;
mod scenes;
mod ui;
mod word_list;

use crate::scenes::game::GameScene;
use crate::scenes::menu::MenuScene;
use anyhow::Result;
use log::LevelFilter;
use pixels_graphics_lib::prelude::*;

const WIDTH: usize = 260;
const HEIGHT: usize = 300;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Input {
    Action,
    Up,
    Down,
    Left,
    Right,
    Escape,
}

fn main() -> Result<()> {
    setup_logger();

    start_menu()?;

    Ok(())
}

fn setup_logger() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Warn)
        .filter_module("wordle", LevelFilter::Trace)
        .init();
}

fn start_menu() -> Result<(), GraphicsError> {
    let switcher: SceneSwitcher<SceneResult, SceneName> = |_, list, name| match name {
        SceneName::Game(word_size) => list.push(GameScene::new(word_size)),
    };

    let menu = MenuScene::new();

    run_scenes(
        WIDTH,
        HEIGHT,
        "Wordle",
        Some(WindowPreferences::new("app", "emmabritton", "wordle_menu", 1).unwrap()),
        switcher,
        menu,
        Options::default(),
        empty_pre_post(),
    )
}

#[derive(Debug, Clone, PartialEq)]
enum SceneName {
    Game(usize),
}

#[derive(Debug, Clone, PartialEq)]
enum SceneResult {}
