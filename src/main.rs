use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {

    let mut player_pos = vec2(0.0, screen_height() - 180.0);
    let player_speed = 200.0;
    let mut player_velocity = vec2(0.0, 0.0);

    let platforms = [Rect::new(0.0, screen_height() - 100.0, screen_width(), 100.0)];

    loop {
        clear_background(BLUE);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(0.0, screen_height() - 100.0, screen_width(), 100.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        
        if is_key_down(KeyCode::Right)   {player_velocity.x = player_speed}
        else if is_key_down(KeyCode::Left)    {player_velocity.x = -player_speed}
        else {player_velocity = vec2(0.0, 0.0)};

        if is_key_pressed(KeyCode::Space) {
            if player_pos.y == screen_height() - 180.0 {
                println!("jnbjfgiuut");
            }
        }

        for platform in platforms {
            draw_rectangle(platform.x, platform.y, platform.w, platform.h, GREEN)
        }
        draw_rectangle(player_pos.x, player_pos.y, 20.0, 80.0, BLACK);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        player_pos += player_velocity * get_frame_time();

        next_frame().await
    }
}