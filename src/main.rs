use winit::{application::ApplicationHandler, error::EventLoopError, event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowId}};
use std::sync::Arc;

pub mod num;
pub mod num2;

pub struct App {
    window: Option<Arc<Window>>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                println!("{event:?}");
            },
            _ => {},
        }
    }
}

impl App {
    pub fn new() -> Self {
        App {
            window:None
        }
    }

    pub fn render(&mut self) {
        if let Some(ref window) = self.window { window.request_redraw(); }
    }
}



fn main() -> Result<(), EventLoopError> {
    env_logger::init();
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app)
}

