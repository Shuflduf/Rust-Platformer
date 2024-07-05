use macroquad::math::{vec2, Rect, Vec2};

pub struct Level {
    pub name: &'static str,
    pub start_pos: Vec2,
    pub platforms: Vec<Rect>,
    pub finish: Vec2,
}
pub fn level_0() -> Level {
    Level{
        name: "LEVEL ZERO",
        start_pos: vec2(350.0, 40.0),
        platforms: vec![
            Rect::new(300.0, 50.0, 100.0, 20.0),
            Rect::new(200.0, 150.0, 100.0, 20.0),
            Rect::new(100.0, 250.0, 100.0, 20.0),
            Rect::new(0.0, 350.0, 100.0, 20.0),
        ],
        finish: vec2(25.0, 300.0),
        }
    }

pub fn level_1() -> Level {
    Level{
        name: "LEVEL ONE",
        start_pos: vec2(20.0, 300.0),
        platforms: vec![
            Rect::new(0.0, 350.0, 400.0, 50.0),
            Rect::new(100.0, 290.0, 100.0, 20.0),
            Rect::new(150.0, 230.0, 100.0, 20.0),
            Rect::new(200.0, 160.0, 200.0, 20.0),
            Rect::new(40.0, 80.0, 200.0, 20.0),
        ],
        finish: vec2(50.0, 30.0)
    }
}

