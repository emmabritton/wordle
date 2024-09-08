use crate::Input;
use pixels_graphics_lib::prelude::*;

pub mod game;
pub mod menu;

fn keys_to_input(keys: &FxHashSet<KeyCode>, controller: &GameController) -> Option<Input> {
    if controller.direction.up || keys.contains(&KeyCode::ArrowUp) {
        Some(Input::Up)
    } else if controller.direction.down || keys.contains(&KeyCode::ArrowDown) {
        Some(Input::Down)
    } else if controller.direction.right || keys.contains(&KeyCode::ArrowRight) {
        Some(Input::Right)
    } else if controller.direction.left || keys.contains(&KeyCode::ArrowLeft) {
        Some(Input::Left)
    } else if controller.action.south || keys.contains(&KeyCode::Enter) {
        Some(Input::Action)
    } else if controller.action.east || keys.contains(&KeyCode::Escape) {
        Some(Input::Escape)
    } else {
        None
    }
}
