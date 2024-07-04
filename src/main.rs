use macroquad::prelude::*;

struct Player {
    position: Vec2,
    speed: f32,
    velocity: Vec2,
}
impl Player {
    fn on_ground(&self) -> bool {
        // println!("{}", self.position.y > screen_height() - 180.0);
        self.position.y > screen_height() - 180.0
    }
}


#[macroquad::main("BasicShapes")]
async fn main() {

    let mut player = Player{
        position: vec2(0.0, screen_height() - 180.0),
        speed: 200.0,
        velocity: vec2(0.0, 0.0)
    };


    let platforms = [Rect::new(0.0, screen_height() - 100.0, screen_width(), 100.0)];

    loop {
        clear_background(BLUE);

        draw_rectangle(0.0, screen_height() - 100.0, screen_width(), 100.0, GREEN);
        
        if is_key_down(KeyCode::Right)   {player.velocity.x = player.speed * get_frame_time()}
        else if is_key_down(KeyCode::Left)    {player.velocity.x = -player.speed * get_frame_time()}
        else {player.velocity.x = 0.0};

        if player.on_ground() {
            if is_key_pressed(KeyCode::Space){
                player.velocity.y = -4.0;
            } else {
                player.velocity.y = 0.0;
                player.position.y = screen_height() - 179.0;
            }
        } else {
            player.velocity.y += 10.0 * get_frame_time()
        }

        for platform in platforms {
            draw_rectangle(platform.x, platform.y, platform.w, platform.h, GREEN)
        }
        draw_rectangle(player.position.x, player.position.y, 20.0, 80.0, BLACK);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        player.position += player.velocity;

        next_frame().await
    }
}