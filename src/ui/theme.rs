pub mod colors {
    use pixels_graphics_lib::prelude::{Color, LIGHT_GRAY};

    const WHITE: Color = Color::new(230, 230, 230, 255);
    const GREEN: Color = Color::new(50, 170, 50, 255);
    const RED: Color = Color::new(170, 50, 50, 255);
    const YELLOW: Color = Color::new(180, 180, 40, 255);
    const GREY: Color = Color::new(180, 180, 180, 255);
    const DARK_GREY: Color = Color::new(100, 100, 100, 255);
    const BLACK: Color = Color::new(15, 15, 15, 255);

    pub const BACKGROUND: Color = WHITE;
    pub const ERROR: Color = RED;
    pub const KEYBOARD_BACK: Color = GREY;
    pub const KEYBOARD_HIGHLIGHT: Color = DARK_GREY;
    pub const KEYBOARD_LETTER: Color = BLACK;
    pub const SLOT_EMPTY_BORDER: Color = GREY;
    pub const SLOT_POS_WRONG_BACK: Color = YELLOW;
    pub const SLOT_POS_WRONG_FORE: Color = WHITE;
    pub const SLOT_POS_RIGHT_BACK: Color = GREEN;
    pub const SLOT_POS_RIGHT_FORE: Color = WHITE;
    pub const SLOT_NO_MATCH_BACK: Color = DARK_GREY;
    pub const SLOT_NO_MATCH_FORE: Color = WHITE;
    pub const SLOT_GUESS_LETTER: Color = BLACK;
    pub const SLOT_GUESS_BORDER: Color = DARK_GREY;
    pub const MENU_DEFAULT: Color = GREY;
    pub const MENU_SELECTED: Color = BLACK;
    pub const BUTTON_BAR: Color = GREY;

    pub const WIN_BACK: Color = LIGHT_GRAY;
    pub const WIN_BANNER: Color = GREEN;
    pub const WIN_TEXT: Color = BLACK;

    pub const LOSE_BACK: Color = LIGHT_GRAY;
    pub const LOSE_BANNER: Color = RED;
    pub const LOSE_TEXT: Color = BLACK;
}
