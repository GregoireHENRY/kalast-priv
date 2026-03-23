use std::sync::Arc;

pub struct App {
    pub state: Option<crate::app::state::State>,
    pub config: Arc<crate::app::config::Config>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: None,
            config: Arc::new(crate::app::config::Config::default()),
        }
    }

    pub fn new_with_config(config: crate::app::config::Config) -> Self {
        Self {
            state: None,
            config: Arc::new(config),
        }
    }
}

impl winit::application::ApplicationHandler<crate::app::state::State> for App {
    fn resumed(&mut self, ev: &winit::event_loop::ActiveEventLoop) {
        let size = winit::dpi::PhysicalSize::new(self.config.width, self.config.height);
        let attrs = winit::window::Window::default_attributes().with_inner_size(size);

        let win = Arc::new(ev.create_window(attrs).unwrap());

        self.state = Some(pollster::block_on(crate::app::state::State::new(
            ev.owned_display_handle(),
            win.clone(),
            self.config.clone(),
        )));
        // win.request_redraw();
    }

    fn window_event(
        &mut self,
        ev: &winit::event_loop::ActiveEventLoop,
        _id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();

        match event {
            winit::event::WindowEvent::CloseRequested => ev.exit(),
            winit::event::WindowEvent::Resized(size) => state.resize(size),
            winit::event::WindowEvent::RedrawRequested => {
                state.update();
                state.render();
                state.get_window().request_redraw();
            }
            winit::event::WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => crate::app::input::handle_key(ev, code, key_state.is_pressed()),
            _ => {}
        };
    }
}
