use crate::{Color, Context, MouseButton};
use num_traits::{AsPrimitive, FromPrimitive};

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
        unsafe {
            #[allow(static_mut_refs)]
            CONTEXT.take(); // drop if any
        }
        default_hook(info);
    }));
}

macro_rules! numify {
    ($($i:ident),*) => {
        $(let $i = $i.as_();)*
    };
}

pub fn set_pixel<N>(x: N, y: N, color: Color)
where
    N: AsPrimitive<u16>,
{
    numify!(x, y);
    ctx().set_pixel(x, y, color);
}

pub fn begin_drawing() {
    ctx().begin_drawing();
}

pub fn next_frame() {
    ctx().next_frame();
}

pub fn fill_rect<X, Y, W, H>(x: X, y: Y, width: W, height: H, color: Color)
where
    X: AsPrimitive<u16>,
    Y: AsPrimitive<u16>,
    W: AsPrimitive<u16>,
    H: AsPrimitive<u16>,
{
    numify!(x, y, width, height);
    ctx().fill_rect(x, y, width, height, color);
}

pub fn screen_width<T>() -> T
where
    T: FromPrimitive,
{
    T::from_u16(ctx().screen_width()).unwrap()
}

pub fn screen_height<T>() -> T
where
    T: FromPrimitive,
{
    T::from_u16(ctx().screen_height()).unwrap()
}

pub fn mouse_position<X, Y>() -> (X, Y)
where
    X: FromPrimitive,
    Y: FromPrimitive,
{
    let (x, y) = ctx().mouse_position();
    (X::from_u16(x).unwrap(), Y::from_u16(y).unwrap())
}

pub fn clear_background(color: Color) {
    ctx().clear_background(color);
}

pub fn exit_app() {
    unsafe {
        #[allow(static_mut_refs)]
        CONTEXT.take(); // drop if any
    }
    std::process::exit(0);
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

pub fn mouse_positions<'a>() -> &'a [(u16, u16)] {
    ctx().mouse_positions()
}

pub fn set_target_fps(fps: u16) {
    ctx().set_target_fps(fps);
}
