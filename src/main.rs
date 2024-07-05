// use levels::levels::{level_1, level_2};
use macroquad::{input::KeyCode, prelude::*};
use miniquad::window::{set_window_position, set_window_size};

use crate::levels::levels::*;
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

    fn in_end(&self, finish: &Rect) -> bool {
        let center = self.position + (self.size / 2.0);
        center.x > finish.x && center.x < finish.x + finish.w && center.y > finish.y && center.y < finish.y + finish.h
    }
}

#[macroquad::main("Platformer")]
async fn main() {

    let all_levels = [level_0(), level_1()];
    let current_level = 0;

    let mut player = Player{
        position: vec2(0.0, 0.0),
        size: vec2(20.0, 20.0),
        jump: 3.0,
        speed: 100.0,
        velocity: vec2(0.0, 0.0)
    };
    
    set_window_size(400, 400);
    let window_multiplier = vec2(4.0, 1.97);

    let current_stage = &all_levels[current_level];
    player.position = current_stage.start_pos;


    loop {
        clear_background(BLUE);

        
       
        player.velocity.x = get_input_dir(KeyCode::Left, KeyCode::Right) * player.speed * get_frame_time();
        // let platforms_slive = &current_level[..];
        if player.on_ground(current_stage.platforms.clone()) {
            if is_key_pressed(KeyCode::Space){
                player.velocity.y = -player.jump;
            } else {
                player.velocity.y = 0.0;
            }
        } else {
            player.velocity.y += 10.0 * get_frame_time()
        }

        for platform in &current_stage.platforms {
            draw_rectangle(platform.x, platform.y, platform.w, platform.h, GREEN)
        }
        draw_rectangle(current_stage.finish.x, current_stage.finish.y, current_stage.finish.w, current_stage.finish.h, RED);

        

        draw_rectangle(player.position.x, player.position.y, player.size.x, player.size.y, BLACK);

        draw_text("LEVEL ONE", 20.0, 20.0, 30.0, DARKGRAY);

        set_window_position((player.position.x * window_multiplier.x) as u32 , (player.position.y * window_multiplier.y) as u32);
        player.position += player.velocity;
        println!("{}", player.in_end(&current_stage.finish));

        if player.position.y > 1080.0 {
            player.position = current_stage.start_pos
        }

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