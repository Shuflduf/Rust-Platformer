pub mod levels {
    use macroquad::math::Rect;

    pub fn level_1() -> [Rect; 2] {
        let level = [
            Rect::new(0.0, 350.0, 400.0, 50.0),
            Rect::new(200.0, 400.0, 300.0, 50.0)
        ];
        level
    }
}