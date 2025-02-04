use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};

pub struct InputController {
    enigo: Enigo,
}

#[allow(dead_code)]
impl InputController {
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(&Settings::default()).unwrap(),
        }
    }

    pub fn click_left_button(&mut self) {
        self.enigo.button(Button::Left, Direction::Click).unwrap();
    }

    pub fn click_right_button(&mut self) {
        self.enigo.button(Button::Right, Direction::Click).unwrap();
    }

    pub fn mouse_move_to(&mut self, x: i32, y: i32) {
        self.enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
    }
} 