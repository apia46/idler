use winit::{application::ApplicationHandler, error::EventLoopError, event::{ElementState, MouseButton, WindowEvent}, event_loop::{ActiveEventLoop, EventLoop}, window::WindowId};

use crate::Game;

pub struct App {
    event_loop: EventLoop<()>,
    app_handler: AppHandler,
}

impl App {
    pub fn new() -> Self {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        let app_handler = AppHandler::default();
        App { event_loop, app_handler }
    }

    pub fn run(mut self) -> Result<(), EventLoopError> {
        self.event_loop.run_app(&mut self.app_handler)
    }
}

#[derive(Default)]
struct AppHandler {
    game: Option<Game>
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        self.game = Some(pollster::block_on(Game::new(event_loop)));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match (event, &mut self.game) {
            (WindowEvent::CloseRequested, _) => event_loop.exit(),
            (WindowEvent::Resized(size), Some(game)) => game.window.resize(size),
            (WindowEvent::KeyboardInput { device_id:_ , event:_, is_synthetic:_ }, _) => {
            },
            (WindowEvent::MouseInput { device_id:_, state:ElementState::Pressed, button:MouseButton::Left }, Some(game)) => {
                game.render_state.things_per_second += game.render_state.things;
                game.render_state.things = 0.into();
            },
            (WindowEvent::RedrawRequested, Some(game)) => {
                game.window.render(|| game.renderer.draw_scene(&mut game.render_state));
            },
            _ => {},
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let Some(game) = &self.game else { return; };
        if (game.render_state.start_time.elapsed() - game.render_state.previous_frame).as_secs_f32() > 1./60. {
            game.window.request_redraw();
        }
    }
}

