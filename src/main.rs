use macroquad::{prelude::*, input::KeyCode};

struct Player {
    position: Vec2,
    size: Vec2,
    speed: f32,
    velocity: Vec2,
} impl Player {
    fn on_ground(&mut self, platforms: &[Rect]) -> bool {
        for i in platforms {
            
            if self.position.y + self.size.y >= i.y && (self.position.y + self.size.y < i.y + i.h) {
                if (self.position.x + self.size.x / 2.0) > i.x && (self.position.x + self.size.x / 2.0) < i.x + i.w{
                    self.position.y = i.y - self.size.y; // causes teleporting
                    return true;
                }
            } 
        }
        false
    }
}

#[macroquad::main("Platformer")]
async fn main() {

    let mut player = Player{
        position: vec2(0.0, 400.0),
        size: vec2(20.0, 80.0),
        speed: 200.0,
        velocity: vec2(0.0, 0.0)
    };


    let platforms = [
        Rect::new(0.0, screen_height() - 100.0, screen_width(), 100.0),
        Rect::new(200.0, 400.0, 300.0, 50.0)
    ];

    // set_fullscreen(true);

    loop {
        clear_background(BLUE);

       
        player.velocity.x = get_input_dir(KeyCode::Left, KeyCode::Right) * player.speed * get_frame_time();

        if player.on_ground(&platforms) {
            if is_key_pressed(KeyCode::Space){
                player.velocity.y = -4.0;
                println!("jump");
            } else {
                player.velocity.y = 0.0;
                // player.position.y = screen_height() - 179.0;
            }
        } else {
            player.velocity.y += 10.0 * get_frame_time()
        }

        for platform in platforms {
            draw_rectangle(platform.x, platform.y, platform.w, platform.h, GREEN)
        }

        

        draw_rectangle(player.position.x, player.position.y, player.size.x, player.size.y, BLACK);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        player.position += player.velocity;
        
        next_frame().await
    }
}

fn get_input_dir(first: KeyCode, second: KeyCode) -> f32 {
    if is_key_down(first) && is_key_down(second) {
        return 0.0
    } else if is_key_down(first) {
        return -1.0
    } else if is_key_down(second){
        return 1.0;
    } else {
        0.0
    }
}

// fn move_player(dir: vec2) {
//  pass
// }