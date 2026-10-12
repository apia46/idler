use winit::{application::ApplicationHandler, event_loop::ActiveEventLoop, window::Window};
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
}

impl App {
    pub async fn new() -> Self {
        App {
            window:None
        }
    }

    pub fn render(&mut self) {
        if let Some(ref window) = self.window { window.request_redraw(); }
    }
}



fn main() {
    
}

