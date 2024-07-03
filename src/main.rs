use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {

    let mut player_pos = 0.0;
    let player_speed = 200.0;

    loop {
        clear_background(BLUE);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(0.0, screen_height() - 100.0, screen_width(), 100.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        
        if is_key_down(KeyCode::Right)   {player_pos += player_speed * get_frame_time()};
        if is_key_down(KeyCode::Left)    {player_pos -= player_speed * get_frame_time()}

        draw_rectangle(player_pos, screen_height() - 180.0, 20.0, 80.0, BLACK);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}