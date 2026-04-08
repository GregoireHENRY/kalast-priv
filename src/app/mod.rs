pub mod body;
pub mod camera;
pub mod config;
pub mod gpu;
pub mod pass;
pub mod simulation;
pub mod uniform;
pub mod window;

use pyo3::prelude::*;
use std::sync::Arc;

use crate::Float;

pub struct App {
    pub config: crate::app::config::Config,
    pub window: Option<crate::app::window::Window>,

    pub now: std::time::Instant,
    pub dt: Float,

    pub simulation: crate::app::simulation::Simulation,
    pub tick: Option<Tick>,

    pub controller: camera::Controller,
}

impl App {
    pub fn new() -> Self {
        Self::new_with_config(crate::app::config::Config::default())
    }

    pub fn new_with_config(config: crate::app::config::Config) -> Self {
        let controller = camera::Controller::new(
            config.sensitivity_move,
            config.sensitivity_look,
            config.sensitivity_rotate,
            config.sensitivity_zoom,
        );

        Self {
            config,
            window: None,

            now: std::time::Instant::now(),
            dt: 0.0,

            simulation: crate::app::simulation::Simulation::new(),
            tick: None,

            controller,
        }
    }

    pub fn start(&mut self) {
        env_logger::init();
        let ev = winit::event_loop::EventLoop::with_user_event()
            .build()
            .unwrap();

        ev.run_app(self).unwrap();
    }

    pub fn set_tick<F>(&mut self, f: F)
    where
        F: Fn(&mut simulation::Simulation) + 'static,
    {
        self.tick = Some(Tick::Rust(Box::new(f)));
    }

    pub fn with_tick<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut simulation::Simulation) + 'static,
    {
        self.set_tick(f);
        self
    }

    pub fn exit(&self, ev: &winit::event_loop::ActiveEventLoop) {
        let win = self.window.as_ref().unwrap();

        if self.simulation.camera.control == camera::Control::WASD {
            win.reset_cursor();
        }

        ev.exit()
    }
}

impl winit::application::ApplicationHandler<crate::app::window::Window> for crate::app::App {
    fn resumed(&mut self, ev: &winit::event_loop::ActiveEventLoop) {
        let size = winit::dpi::PhysicalSize::new(self.config.width, self.config.height);
        let attrs = winit::window::Window::default_attributes()
            .with_inner_size(size)
            .with_title(&self.config.title);

        let win = Arc::new(ev.create_window(attrs).unwrap());

        self.window = Some(pollster::block_on(crate::app::window::Window::new(
            ev.owned_display_handle(),
            win.clone(),
            &self.config,
            &self.simulation,
        )));
    }

    fn window_event(
        &mut self,
        ev: &winit::event_loop::ActiveEventLoop,
        _id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => self.exit(ev),
            winit::event::WindowEvent::Resized(size) => {
                let win = self.window.as_mut().unwrap();
                win.resize(size.width, size.height, &self.config);
            }
            winit::event::WindowEvent::RedrawRequested => {
                let now = std::time::Instant::now();
                self.dt = (now - self.now).as_secs_f64() as _;
                self.now = now;

                match &self.tick {
                    Some(Tick::Rust(f)) => {
                        f(&mut self.simulation);
                    }
                    Some(Tick::Python {
                        callback,
                        simulation,
                    }) => {
                        Python::attach(|py| {
                            callback.call1(py, (simulation.clone(),)).unwrap();
                        });
                    }
                    None => {}
                };

                {
                    self.simulation
                        .camera
                        .update_with_controller(&mut self.controller, self.dt);

                    self.simulation.update();
                }

                {
                    let win = self.window.as_mut().unwrap();
                    win.update(&mut self.simulation);
                    win.render(&self.config);
                }

                if self.config.debug_app {
                    // println!("[APP][WindowEvent::RedrawRequested] hello");
                }
            }

            winit::event::WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                let is_pressed = key_state.is_pressed();
                self.controller.handle_key(code, is_pressed);

                match (code, is_pressed) {
                    (winit::keyboard::KeyCode::Escape, true) => self.exit(ev),
                    (winit::keyboard::KeyCode::Space, true) => {
                        // let win = self.window.as_mut().unwrap();
                        // win.toggle_color_xy = !win.toggle_color_xy;
                    }

                    (winit::keyboard::KeyCode::KeyT, true) => {
                        // switch camera type
                        self.simulation.camera.toggle_control();
                        let control = self.simulation.camera.control;
                        if self.config.debug_app {
                            println!("[APP] Camera control changed, now is {:?}", control);
                        }
                        match control {
                            camera::Control::Arcball => {
                                // reset cursor middle
                                let win = self.window.as_ref().unwrap();
                                win.reset_cursor();
                            }
                            camera::Control::WASD => {
                                // no cursor in WASD
                                let win = self.window.as_ref().unwrap();
                                win.center_cursor();
                                win.window.set_cursor_visible(false);
                                win.window
                                    .set_cursor_grab(winit::window::CursorGrabMode::Confined)
                                    .or_else(|_e| {
                                        win.window
                                            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
                                    })
                                    .unwrap();
                            }
                        }
                    }

                    _ => {}
                };
            }

            winit::event::WindowEvent::PinchGesture { delta, .. } => {
                if self.simulation.camera.control == camera::Control::Arcball {
                    self.controller.zoom(delta as Float);
                }
            }

            winit::event::WindowEvent::MouseInput {
                state: _state,
                button: _button,
                ..
            } => {}

            _ => {}
        };
    }

    fn device_event(
        &mut self,
        _ev_loop: &winit::event_loop::ActiveEventLoop,
        _id: winit::event::DeviceId,
        ev: winit::event::DeviceEvent,
    ) {
        match ev {
            winit::event::DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                if self.simulation.camera.control == camera::Control::WASD {
                    self.controller.mouse_motion(dx as Float, dy as Float);
                }
            }

            winit::event::DeviceEvent::MouseWheel { delta } => {
                let (dx, dy) = match delta {
                    winit::event::MouseScrollDelta::LineDelta(dx, dy) => {
                        (dx as Float * 100.0, dy as Float * 100.0)
                    }
                    winit::event::MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition {
                        x,
                        y,
                    }) => (x as Float, y as Float),
                };

                if self.simulation.camera.control == camera::Control::Arcball {
                    self.controller.mouse_motion(-dx, -dy);
                }
            }
            _ => {}
        };
    }
}

pub enum Tick {
    Rust(Box<dyn for<'a> Fn(&'a mut simulation::Simulation)>),
    Python {
        callback: Py<PyAny>,
        simulation: crate::py::app::simulation::Simulation,
    },
}
