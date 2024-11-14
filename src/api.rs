use crate::{Color, Context, Key, MouseButton};

static mut CONTEXT: Option<Context> = None;
pub fn ctx() -> &'static mut Context {
    #![allow(static_mut_refs)]
    unsafe {
        if CONTEXT.is_none() {
            let _ = CONTEXT.insert(Context::new());
            setup_panic_hook();
        }
        CONTEXT.as_mut().unwrap()
    }
}

pub fn setup_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        exit_app(true);
        default_hook(info);
    }));
}

pub fn set_pixel(x: f32, y: f32, color: Color) {
    ctx().set_pixel(x, y, color);
}

pub fn next_frame() {
    ctx().next_frame();
}

pub fn fill_rect(x: f32, y: f32, width: f32, height: f32, color: Color) {
    ctx().fill_rect(x, y, width, height, color);
}

pub fn screen_width() -> f32 {
    ctx().screen_width()
}

pub fn screen_height() -> f32 {
    ctx().screen_height()
}

pub fn mouse_position() -> (f32, f32) {
    ctx().mouse_position()
}

pub fn clear_background(color: Color) {
    ctx().clear_background(color);
}

pub fn exit_app(panic: bool) {
    let mut hook = None;
    unsafe {
        #[allow(static_mut_refs)]
        if let Some(context) = CONTEXT.take().as_mut() {
            if let Some(exit_hook) = context.exit_hook.take() {
                hook = Some(exit_hook);
            }
        };
    }

    if let Some(hook) = hook {
        hook(panic);
    }
    if !panic {
        std::process::exit(0);
    }
}

pub fn is_mouse_button_down(button: MouseButton) -> bool {
    ctx().is_mouse_button_down(button)
}

pub fn is_mouse_button_pressed(button: MouseButton) -> bool {
    ctx().is_mouse_button_pressed(button)
}

pub fn is_mouse_button_released(button: MouseButton) -> bool {
    ctx().is_mouse_button_released(button)
}

pub fn mouse_positions<'a>() -> &'a [(f32, f32)] {
    ctx().mouse_positions()
}

pub fn set_target_fps(fps: f32) {
    ctx().set_target_fps(fps);
}

pub fn is_key_down(key: Key) -> bool {
    ctx().is_key_down(key)
}

pub fn is_key_pressed(key: Key) -> bool {
    ctx().is_key_pressed(key)
}

pub fn is_key_released(key: Key) -> bool {
    ctx().is_key_released(key)
}

pub fn set_exit_key_combo<I>(keys: I)
where
    I: IntoIterator<Item = Key>,
{
    ctx().set_exit_key_combo(keys);
}

pub fn set_exit_hook<F>(hook: F)
where
    F: FnOnce(bool) + 'static,
{
    ctx().set_exit_hook(hook);
}

pub fn rng<T>(min: T, max: T) -> T
where
    T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform,
{
    ctx().rng(min, max)
}
