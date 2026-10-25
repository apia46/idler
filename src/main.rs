use winit::error::EventLoopError;

use game::Game;
use window::Window;
use renderer::Renderer;

/*
 * EventLoop
 * runs the ApplicationHandler (always exists when eventloop does)
 * handles the App (doesnt always exist)
 * contains:
 * - winit and wgpu and vello stuff
 * - rendering
 * - game state
 */

mod game;
mod window;
mod renderer;

pub mod num;
pub mod num2;


struct App {
    window: Window,
    renderer: Renderer,
}

fn main() -> Result<(), EventLoopError> {
    env_logger::init();
    Game::new().run()
}

