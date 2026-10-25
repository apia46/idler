use pollster::block_on;
use winit::{application::ApplicationHandler, error::EventLoopError, event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, window::WindowId};

use crate::{App, renderer::Renderer, window::Window};

pub struct Game {
    event_loop: EventLoop<()>,
    app_handler: AppHandler,
}

impl Game {
    pub fn new() -> Self {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        let app_handler = AppHandler::default();
        Game { event_loop, app_handler }
    }

    pub fn run(mut self) -> Result<(), EventLoopError> {
        self.event_loop.run_app(&mut self.app_handler)
    }
}

#[derive(Default)]
struct AppHandler {
    app: Option<App>
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.app = Some(block_on((async || App {
            window: Window::new(event_loop).await,
            renderer: Renderer::new(),
        })()));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match (event, &mut self.app) {
            (WindowEvent::CloseRequested, _) => event_loop.exit(),
            (WindowEvent::Resized(size), Some(app)) => app.window.resize(size),
            (WindowEvent::KeyboardInput { device_id:_ , event, is_synthetic:_ }, _) => {
                println!("{event:?}");
            },
            (WindowEvent::RedrawRequested, Some(app)) => {
                app.window.render(app.renderer.draw_scene());
            },
            _ => {},
        }
    }
}

