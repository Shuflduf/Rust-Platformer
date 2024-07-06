use macroquad::{input::KeyCode, prelude::*};
use miniquad::window::{set_window_position, set_window_size};

use crate::levels::*;
mod levels;
mod menus;
struct Player {
    position: Vec2,
    size: Vec2,
    jump: f32,
    speed: f32,
    velocity: Vec2,
} impl Player {
    fn on_ground(&mut self, platforms: &Vec<(Rect, bool)>) -> bool {
        for (i, _passable) in platforms {
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

    fn in_ceiling(&mut self, platforms: &Vec<(Rect, bool)>) {
        for (i, passable) in platforms {
            if *passable {
                continue;
            }
            if self.position.y <= i.y + i.h && self.position.y > i.y{
                if self.position.x + self.size.x / 2.0 > i.x && self.position.x + self.size.x / 2.0 < i.x + i.w {
                    self.velocity.y = 0.1;
                }
            }
        }
    }

    fn on_wall(&mut self, platforms: &Vec<(Rect, bool)>) {
        for (i, passable) in platforms {
            if *passable {
                continue;
            }
            
            if self.position.y >= i.y && self.position.y + self.size.y <= i.y + i.h {

                if self.position.x < i.x + i.w && self.position.x > i.x {
                    self.position.x = i.x + i.w;
                } 

                else if self.position.x + self.size.x > i.x && self.position.x < i.x{
                    self.position.x = i.x - self.size.x;
                }
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

#[macroquad::main("Exodus")]
async fn main() {

    enum GameState {
        MainMenu,
        Game(usize),
        EndScreen,
    }

    let mut current_state = GameState::MainMenu;

    let all_levels = [level_0(), level_1(), level_2(), level_3(), level_4(), level_5(), level_6(), level_7(), level_8(), level_9()];

    let mut player = Player{
        position: vec2(0.0, 0.0),
        size: vec2(20.0, 20.0),
        jump: 3.0,
        speed: 100.0,
        velocity: vec2(0.0, 0.0)
    };

    let clock_enabled = true;
    let mut clock = 0.0;

    set_window_size(400, 400);
    let window_multiplier = vec2(4.0, 1.97);

    loop {
    match current_state {
        GameState::MainMenu => {
            let main_menu = menus::start_menu();

            let mut window_pos = vec2(0.0, 0.0);
            let mut window_dir = vec2(1.0, 1.0);
            let window_size = vec2(400.0, 400.0);

            'render_loop: loop {
                window_pos += window_dir;

                {
                    if window_pos.x <= 0.0 {window_dir.x *= -1.0}
                    if window_pos.y <= 0.0 {window_dir.y *= -1.0}
                    if window_pos.x + window_size.x >= 1920.0 {window_dir.x *= -1.0}
                    if window_pos.y + window_size.y >= 1080.0 {window_dir.y *= -1.0}
                }

                set_window_size(window_size.x as u32, window_size.x as u32);
                set_window_position(window_pos.x as u32, window_pos.y as u32);

                clear_background(WHITE);

                draw_text(main_menu.text, 100.0, 120.0, 80.0, BLACK);
                
                let button_rect = Rect::new(90.0, 150.0, 220.0, 100.0);

                draw_rectangle(button_rect.x, button_rect.y, button_rect.w, button_rect.h, BLACK);
                draw_text(main_menu.button_text, 100.0, 225.0, 90.0, RED);

                if is_mouse_button_pressed(MouseButton::Left) {
                    if button_rect.contains(mouse_position().into()){
                        current_state = GameState::Game(0);
                        break 'render_loop;
                    }
                }

                next_frame().await
            }
        }
        GameState::Game(level) => {
            let finish_size = vec2(50.0, 50.0);

            let current_stage = &all_levels[level];
            player.position = current_stage.start_pos;

            'platformer_loop: loop {
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
                for (platform, passable) in &current_stage.platforms {
                    if !passable {
                       draw_rectangle(platform.x, platform.y, platform.w, platform.h, BLACK) 
                    } else {
                        draw_rectangle(platform.x, platform.y, platform.w, platform.h, DARKGRAY) 
                    }
                    
                }
        
                // draw finish
                draw_rectangle(current_stage.finish.x, current_stage.finish.y, finish_size.x, finish_size.y, RED);
        
                // write level name
                draw_text(&current_stage.name, 20.0, 20.0, 30.0, BLACK);
        
                if clock_enabled {
                    clock += get_frame_time();
                    let clock_text = &format!("{:.2}", clock)[..];
                    draw_text(clock_text, 320.0, 20.0, 30.0, BLACK);
                }
                
        
                // add velocity to player, basically `move_and_slide()`
                player.position += player.velocity;
        
                // check if the player finished
                if player.in_end(current_stage.finish, finish_size) {
                    current_state = GameState::Game(level + 1);
                    if level + 1 >= all_levels.len() {
                        current_state = GameState::EndScreen
                    }
                    break 'platformer_loop;
                }
        
                // reset the player if they go off the screen
                if player.position.y > 1080.0 {
                    player.position = current_stage.start_pos
                }
        
                player.in_ceiling(&current_stage.platforms);
                player.on_wall(&current_stage.platforms);
        
                // move window to player
                set_window_position((player.position.x * window_multiplier.x) as u32 , (player.position.y * window_multiplier.y) as u32);
        
                //draw player
                draw_rectangle(player.position.x, player.position.y, player.size.x, player.size.y, BLACK);
        
                next_frame().await
            }
        }
        GameState::EndScreen => {
            let end_screen = menus::end_screen();
            'render_loop: loop {
                clear_background(WHITE);
                
                let final_time = &format!("{}", clock)[..];
                draw_text(final_time, 100.0, 120.0, 80.0, BLACK);
                
                let button_rect = Rect::new(90.0, 150.0, 220.0, 100.0);

                draw_rectangle(button_rect.x, button_rect.y, button_rect.w, button_rect.h, BLACK);
                draw_text(end_screen.button_text, 100.0, 225.0, 90.0, RED);

                if is_mouse_button_pressed(MouseButton::Left) {
                    if button_rect.contains(mouse_position().into()){
                        current_state = GameState::Game(0);
                        clock = 0.0;
                        break 'render_loop;
                    }
                }

                next_frame().await
            }
        }
    }
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
