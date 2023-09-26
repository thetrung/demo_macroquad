use macroquad::{prelude::*, input};

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut x = 0.0;
    let mut y = 0.0;
    loop {
        clear_background(RED);

        let pressed = input::get_last_key_pressed();
        let pressed_str = format!("HELLO {:?}", if (pressed != None) {pressed} else {None});
        draw_text(&pressed_str, 20.0, 20.0, 20.0, DARKGRAY);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        // switch 
        if input::is_key_down(KeyCode::Up) { y += 10.0; }
        if input::is_key_down(KeyCode::Down ){ y -= 10.0; }
        if input::is_key_down(KeyCode::Left ){ x += 10.0; }
        if input::is_key_down(KeyCode::Right) { x -= 10.0; }

        draw_circle(screen_width() - x, screen_height() - y, 15.0, YELLOW);



        next_frame().await
    }
}