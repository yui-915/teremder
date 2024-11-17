use teremder::*;

fn main() {
    set_target_fps(60.);
    set_exit_key_combo([Key::Q]);
    clear_background(BLACK);

    let mut color = RED;
    let mut size = 1.;

    loop {
        if is_key_down(Key::LeftControl) {
            if is_key_pressed(Key::Num1) {
                size = 1.;
            } else if is_key_pressed(Key::Num2) {
                size = 2.;
            } else if is_key_pressed(Key::Num3) {
                size = 3.;
            } else if is_key_pressed(Key::Num4) {
                size = 4.;
            } else if is_key_pressed(Key::Num5) {
                size = 5.;
            } else if is_key_pressed(Key::Num6) {
                size = 6.;
            } else if is_key_pressed(Key::Num7) {
                size = 7.;
            } else if is_key_pressed(Key::Num8) {
                size = 8.;
            } else if is_key_pressed(Key::Num9) {
                size = 9.;
            }
        } else if is_key_pressed(Key::Num1) {
            color = RED;
        } else if is_key_pressed(Key::Num2) {
            color = GREEN;
        } else if is_key_pressed(Key::Num3) {
            color = BLUE;
        } else if is_key_pressed(Key::Num4) {
            color = YELLOW;
        } else if is_key_pressed(Key::Num5) {
            color = PURPLE;
        } else if is_key_pressed(Key::Num6) {
            color = LIME;
        } else if is_key_pressed(Key::Num7) {
            color = WHITE;
        } else if is_key_pressed(Key::Num8) {
            color = GRAY;
        } else if is_key_pressed(Key::Num9) {
            color = BLACK;
        }

        if is_mouse_button_down(MouseButton::Left) {
            for &(x, y) in mouse_positions() {
                fill_circle(x, y, size + 0.1, color);
            }
        }

        next_frame();
    }
}
