use macroquad::math::{vec2, Rect, Vec2};

pub struct Level {
    pub name: &'static str,
    pub start_pos: Vec2,
    pub platforms: Vec<(Rect, bool)>,
    // if true platform is passable
    pub finish: Vec2,
}
pub fn level_0() -> Level {
    Level{
        name: "YOU'VE ESCAPED",
        start_pos: vec2(350.0, 40.0),
        platforms: vec![
            (Rect::new(300.0, 50.0, 100.0, 20.0), false),
            (Rect::new(200.0, 150.0, 100.0, 20.0), false),
            (Rect::new(100.0, 250.0, 100.0, 20.0), false),
            (Rect::new(0.0, 350.0, 100.0, 20.0), false),
        ],
        finish: vec2(0.0, 300.0),
        }
    }

pub fn level_1() -> Level {
    Level{
        name: "THE GREY VOID",
        start_pos: vec2(20.0, 300.0),
        platforms: vec![
            (Rect::new(0.0, 350.0, 400.0, 50.0), false),
            (Rect::new(100.0, 290.0, 100.0, 20.0), false),
            (Rect::new(150.0, 230.0, 100.0, 20.0), false),
            (Rect::new(200.0, 160.0, 200.0, 20.0), false),
            (Rect::new(40.0, 80.0, 200.0, 20.0), false),
        ],
        finish: vec2(0.0, 0.0)
    }
}

pub fn level_2() -> Level {
    Level{
        name: "THROUGH SHORTCUTS",
        start_pos: vec2(20.0, 20.0),
        platforms: vec![
            (Rect::new(0.0, 40.0, 380.0, 20.0), false),
            (Rect::new(20.0, 80.0, 380.0, 20.0), false),
            (Rect::new(0.0, 120.0, 380.0, 20.0), false),
            (Rect::new(20.0, 160.0, 380.0, 20.0), false),
            (Rect::new(0.0, 200.0, 380.0, 20.0), false),
            (Rect::new(20.0, 240.0, 380.0, 60.0), false),
            (Rect::new(0.0, 350.0, 400.0, 50.0), false),
            
        ],
        finish: vec2(350.0, 300.0)
    }
}

pub fn level_3() -> Level {
    Level{
        name: "AND THROUGH DOORS",
        start_pos: vec2(360.0, 330.0),
        platforms: vec![
            (Rect::new(0.0, 350.0, 400.0, 50.0), false),
            (Rect::new(0.0, 300.0, 50.0, 20.0), true),
            (Rect::new(0.0, 250.0, 50.0, 20.0), true),
            (Rect::new(0.0, 200.0, 50.0, 20.0), true),
            (Rect::new(0.0, 150.0, 50.0, 20.0), true),
            (Rect::new(0.0, 100.0, 50.0, 20.0), true),
            (Rect::new(0.0, 50.0, 50.0, 20.0), true),
            (Rect::new(100.0, 50.0, 50.0, 20.0), true),
            (Rect::new(200.0, 50.0, 50.0, 20.0), true),
            (Rect::new(300.0, 50.0, 50.0, 20.0), true),
        ],
        finish: vec2(350.0, 0.0)
    }
}

pub fn level_4() -> Level {
    Level{
        name: "ESCAPING BOUNDARIES",
        start_pos: vec2(360.0, 40.0),
        platforms: vec![
            (Rect::new(0.0, 350.0, 400.0, 50.0), false),
            (Rect::new(0.0, 110.0, 400.0, 20.0), false),
            (Rect::new(190.0, 0.0, 20.0, 130.0), false),
            (Rect::new(190.0, 200.0, 20.0, 200.0), false),
            (Rect::new(250.0, 270.0, 100.0, 20.0), true),
            (Rect::new(50.0, 270.0, 100.0, 20.0), true),
            (Rect::new(0.0, 190.0, 100.0, 20.0), true),

        ],
        finish: vec2(0.0, 0.0)
    }
}

pub fn level_5() -> Level {
    Level{
        name: "BEFORE COMING BACK",
        start_pos: vec2(20.0, 20.0),
        platforms: vec![
            (Rect::new(0.0, 50.0, 200.0, 20.0), false),
            (Rect::new(190.0, 0.0, 20.0, 70.0), false),
            (Rect::new(200.0, 50.0, 190.0, 20.0), false),
            (Rect::new(380.0, 0.0, 20.0, 70.0), false),
        ],
        finish: vec2(350.0, 350.0)
    }
}

pub fn level_6() -> Level {
    Level{
        name: "TO REPEAT",
        start_pos: vec2(365.0, 350.0),
        platforms: vec![
            (Rect::new(350.0, 350.0, 50.0, 50.0), false),
            (Rect::new(270.0, 270.0, 50.0, 20.0), true),
            (Rect::new(190.0, 190.0, 50.0, 20.0), true),
            (Rect::new(120.0, 190.0, 20.0, 20.0), false),
        ],
        finish: vec2(0.0, 350.0)
    }
}

pub fn level_7() -> Level {
    Level{
        name: "SO WHY",
        start_pos: vec2(10.0, 350.0),
        platforms: vec![
            (Rect::new(0.0, 350.0, 50.0, 50.0), false),
            (Rect::new(40.0, 0.0, 20.0, 400.0), false),
            (Rect::new(0.0, 290.0, 50.0, 20.0), false),
            (Rect::new(0.0, 230.0, 50.0, 20.0), false),
            (Rect::new(0.0, 170.0, 50.0, 20.0), false),
            (Rect::new(0.0, 110.0, 50.0, 20.0), false),
            (Rect::new(0.0, 50.0, 50.0, 20.0), false),
            (Rect::new(120.0, 0.0, 20.0, 250.0), false),
            (Rect::new(85.0, 350.0, 20.0, 20.0), false),
            (Rect::new(120.0, 270.0, 20.0, 20.0), false),
            (Rect::new(160.0, 300.0, 20.0, 20.0), false),
            (Rect::new(230.0, 280.0, 20.0, 20.0), false),
            (Rect::new(260.0, 220.0, 20.0, 20.0), false),
            (Rect::new(260.0, 150.0, 20.0, 20.0), false),
            (Rect::new(320.0, 80.0, 50.0, 20.0), false),
        ],
        finish: vec2(350.0, 0.0)
    }
}

pub fn level_8() -> Level {
    Level{
        name: "DON'T YOU",
        start_pos: vec2(370.0, 40.0),
        platforms: vec![
            (Rect::new(300.0, 60.0, 100.0, 20.0), false),
            (Rect::new(390.0, 0.0, 10.0, 80.0), false),

            // grid start
            (Rect::new(50.0, 150.0, 20.0, 20.0), true),
            (Rect::new(100.0, 150.0, 20.0, 20.0), true),
            (Rect::new(150.0, 150.0, 20.0, 20.0), true),
            (Rect::new(200.0, 150.0, 20.0, 20.0), true),
            (Rect::new(250.0, 150.0, 20.0, 20.0), true),
            (Rect::new(300.0, 150.0, 20.0, 20.0), true),
            (Rect::new(350.0, 150.0, 20.0, 20.0), true),

            (Rect::new(75.0, 200.0, 20.0, 20.0), true),
            (Rect::new(125.0, 200.0, 20.0, 20.0), true),
            (Rect::new(175.0, 200.0, 20.0, 20.0), true),
            (Rect::new(225.0, 200.0, 20.0, 20.0), true),
            (Rect::new(275.0, 200.0, 20.0, 20.0), true),
            (Rect::new(325.0, 200.0, 20.0, 20.0), true),

            (Rect::new(50.0, 250.0, 20.0, 20.0), true),
            (Rect::new(100.0, 250.0, 20.0, 20.0), true),
            (Rect::new(150.0, 250.0, 20.0, 20.0), true),
            (Rect::new(200.0, 250.0, 20.0, 20.0), true),
            (Rect::new(250.0, 250.0, 20.0, 20.0), true),
            (Rect::new(300.0, 250.0, 20.0, 20.0), true),
            (Rect::new(350.0, 250.0, 20.0, 20.0), true),
            // grid end



            (Rect::new(100.0, 330.0, 300.0, 20.0), false),
            (Rect::new(80.0, 380.0, 40.0, 20.0), false),
            (Rect::new(140.0, 380.0, 40.0, 20.0), false),
            (Rect::new(200.0, 380.0, 150.0, 20.0), false),

            (Rect::new(390.0, 330.0, 10.0, 80.0), false),

        ],
        finish: vec2(340.0, 350.0)
    }
}

pub fn level_9() -> Level {
    Level{
        name: "RECOGNIZE ME",
        start_pos: vec2(365.0, 345.0),

        platforms: vec![
            (Rect::new(350.0, 350.0, 50.0, 50.0), false),
            (Rect::new(100.0, 200.0, 300.0, 20.0), false),
            (Rect::new(0.0, 50.0, 400.0, 20.0), false),

            (Rect::new(300.0, 300.0, 20.0, 20.0), false),
            (Rect::new(270.0, 250.0, 20.0, 20.0), false),
            (Rect::new(230.0, 300.0, 20.0, 20.0), false),
            (Rect::new(170.0, 320.0, 20.0, 20.0), false),
            (Rect::new(100.0, 280.0, 30.0, 20.0), false),

            (Rect::new(120.0, 140.0, 40.0, 80.0), false),
            (Rect::new(120.0, 50.0, 40.0, 70.0), false),

            (Rect::new(180.0, 180.0, 40.0, 40.0), false),
            (Rect::new(180.0, 50.0, 40.0, 110.0), false),

            (Rect::new(240.0, 100.0, 20.0, 100.0), true),
            (Rect::new(260.0, 100.0, 20.0, 120.0), false),
            (Rect::new(240.0, 50.0, 40.0, 30.0), false),

            (Rect::new(360.0, 150.0, 40.0, 20.0), true),
            (Rect::new(360.0, 110.0, 40.0, 20.0), true),
        ],
        finish: vec2(0.0, 0.0)
    }
}