use macroquad::{input::KeyCode, prelude::*};
use miniquad::window::{get_window_position, set_window_position, set_window_size};

mod levels;
struct Player {
    position: Vec2,
    size: Vec2,
    jump: f32,
    speed: f32,
    velocity: Vec2,
} impl Player {
    fn on_ground(&mut self, platforms: Vec<Rect>) -> bool {
        for i in platforms {
            if self.position.y + self.size.y >= i.y && self.position.y + self.size.y <= i.y + i.h {
                if (self.position.x + self.size.x / 2.0) > i.x && (self.position.x + self.size.x / 2.0) < i.x + i.w{
                    if self.velocity.y >= 0.0 {
                        self.position.y = i.y - self.size.y;
                        return true;
                    }
                }
            } 
        }
        false
    }
}

#[macroquad::main("Platformer")]
async fn main() {

    let mut player = Player{
        position: vec2(0.0, 0.0),
        size: vec2(20.0, 20.0),
        jump: 3.0,
        speed: 100.0,
        velocity: vec2(0.0, 0.0)
    };
    
    set_window_size(400, 400);
    let window_multiplier = vec2(4.0, 1.97);

    let current_level = levels::levels::level_1();
    player.position = current_level.start_pos;


    loop {
        clear_background(BLUE);

        
       
        player.velocity.x = get_input_dir(KeyCode::Left, KeyCode::Right) * player.speed * get_frame_time();
        // let platforms_slive = &current_level[..];
        if player.on_ground(current_level.platforms.clone()) {
            if is_key_pressed(KeyCode::Space){
                player.velocity.y = -player.jump;
                println!("jump");
            } else {
                player.velocity.y = 0.0;
            }
        } else {
            player.velocity.y += 10.0 * get_frame_time()
        }

        for platform in &current_level.platforms {
            draw_rectangle(platform.x, platform.y, platform.w, platform.h, GREEN)
        }
        draw_rectangle(current_level.finish.x, current_level.finish.y, current_level.finish.w, current_level.finish.h, RED);

        

        draw_rectangle(player.position.x, player.position.y, player.size.x, player.size.y, BLACK);

        draw_text("LEVEL ONE", 20.0, 20.0, 30.0, DARKGRAY);

        println!("{:?}", get_window_position());
        set_window_position((player.position.x * window_multiplier.x) as u32 , (player.position.y * window_multiplier.y) as u32);
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