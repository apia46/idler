use winit::{error::EventLoopError, event_loop::ActiveEventLoop};

use crate::app::App;
use crate::window::Window;
use crate::renderer::{Renderer, RenderState};

mod app;
mod window;
mod renderer;

pub mod num;
pub mod num2;


struct Game {
    window: Window,
    renderer: Renderer,
    render_state: RenderState,
}

impl Game {
    async fn new(event_loop:&ActiveEventLoop) -> Game {
        Game {
            window: Window::new(event_loop).await,
            renderer: Renderer::new(),
            render_state: RenderState::new(),
        }
    }
}

fn main() -> Result<(), EventLoopError> {
    env_logger::init();
    App::new().run()
}

