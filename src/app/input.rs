pub fn handle_key(
    ev: &winit::event_loop::ActiveEventLoop,
    code: winit::keyboard::KeyCode,
    is_pressed: bool,
) {
    match (code, is_pressed) {
        (winit::keyboard::KeyCode::Escape, true) => ev.exit(),
        _ => {}
    };
}


pub fn handle_mouse(
    _ev: &winit::event_loop::ActiveEventLoop,
) {
}