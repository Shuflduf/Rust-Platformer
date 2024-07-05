pub mod levels {
    use macroquad::{math::Rect, window::{screen_height, screen_width}};

    pub fn level_1() -> [Rect; 2] {
        let level = [
            Rect::new(0.0, screen_height() - 100.0, screen_width(), 100.0),
            Rect::new(200.0, 400.0, 300.0, 50.0)
        ];
        level
    }
}