// use levels::levels::{level_1, level_2};
use macroquad::{input::KeyCode, prelude::*};
use miniquad::window::{set_window_position, set_window_size};

use crate::levels::*;
mod levels;
struct Player {
    position: Vec2,
    size: Vec2,
    jump: f32,
    speed: f32,
    velocity: Vec2,
} impl Player {
    fn on_ground(&mut self, platforms: &Vec<Rect>) -> bool {
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

    fn in_ceiling(&mut self, platforms: &Vec<Rect>) {
        for i in platforms {
            if self.position.y <= i.y + i.h && self.position.y > i.y{
                if self.position.x + self.size.x / 2.0 > i.x && self.position.x + self.size.x / 2.0 < i.x + i.w {
                    self.velocity.y = 1.0;
                }
            }
        }
    }

    fn on_wall(&mut self, platforms: &Vec<Rect>) {
        for i in platforms {
            if self.position.y > i.y && self.position.y + self.size.y < i.y + i.h {

                if self.position.x < i.x + i.w{
                    self.position.x = i.x + i.w;
                } 

                // else if self.position.x + self.size.x > i.x {
                //     self.position.x = i.x + self.size.x;
                // }
            }
        }
    }

    fn in_end(&self, finish_pos: Vec2, finish_size: Vec2) -> bool {
        let center = self.position + (self.size / 2.0);
            center.x > finish_pos.x 
            && center.x < finish_pos.x + finish_size.x
            && center.y > finish_pos.y 
            && center.y < finish_pos.y + finish_size.y
    }
}

#[macroquad::main("Platformer")]
async fn main() {

    let all_levels = [level_0(), level_1(), level_2(), level_3()];
    let mut current_level = 3;

    let mut player = Player{
        position: vec2(0.0, 0.0),
        size: vec2(20.0, 20.0),
        jump: 3.0,
        speed: 100.0,
        velocity: vec2(0.0, 0.0)
    };

    let finish_size = vec2(50.0, 50.0);
    
    set_window_size(400, 400);
    let window_multiplier = vec2(4.0, 1.97);

    let mut current_stage = &all_levels[current_level];
    player.position = current_stage.start_pos;


    loop {
        clear_background(WHITE);
        set_window_size(400, 400);

        player.velocity.x = get_input_dir(KeyCode::Left, KeyCode::Right) * player.speed * get_frame_time();

        if player.on_ground(&current_stage.platforms) {
            if is_key_pressed(KeyCode::Space){
                player.velocity.y = -player.jump;
            } else {
                player.velocity.y = 0.0;
            }
        } else {
            player.velocity.y += 10.0 * get_frame_time()
        }

        // draw all platforms
        for platform in &current_stage.platforms {
            draw_rectangle(platform.x, platform.y, platform.w, platform.h, BLACK)
        }

        // draw finish
        draw_rectangle(current_stage.finish.x, current_stage.finish.y, finish_size.x, finish_size.y, RED);

        // write level name
        draw_text(&current_stage.name, 20.0, 20.0, 30.0, BLACK);

        // add velocity to player, basically `move_and_slide()`
        player.position += player.velocity;

        // check if the player finished
        if player.in_end(current_stage.finish, finish_size) {
            current_level += 1;
            current_stage = &all_levels[current_level];
            player.position = current_stage.start_pos;
        }

        // reset the player if they go off the screen
        if player.position.y > 1080.0 {
            player.position = current_stage.start_pos
        }

        // move window to player
        set_window_position((player.position.x * window_multiplier.x) as u32 , (player.position.y * window_multiplier.y) as u32);

        //draw player
        draw_rectangle(player.position.x, player.position.y, player.size.x, player.size.y, BLACK);

        player.in_ceiling(&current_stage.platforms);
        player.on_wall(&current_stage.platforms);

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
