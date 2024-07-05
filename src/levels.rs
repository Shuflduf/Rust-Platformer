pub mod levels {
    use macroquad::math::Rect;

    pub struct Level {
        pub platforms: Vec<Rect>,
        pub finish: Rect,
    }

    pub fn level_1() -> Level {
        Level{
            platforms: vec![
                Rect::new(0.0, 350.0, 400.0, 50.0),
                Rect::new(100.0, 290.0, 100.0, 20.0),
                Rect::new(150.0, 230.0, 100.0, 20.0),
                Rect::new(200.0, 160.0, 200.0, 20.0),
                Rect::new(40.0, 80.0, 200.0, 20.0),
            ],
            finish: Rect::new(50.0, 30.0, 50.0, 50.0),
        }
    }
}