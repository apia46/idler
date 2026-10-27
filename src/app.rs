use std::time::{Duration, Instant};

use winit::{application::ApplicationHandler, error::EventLoopError, event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, keyboard::PhysicalKey, window::WindowId};

use crate::{Game, game};
use crate::input;

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
        let mut game = pollster::block_on(Game::new(event_loop));
        game::test_section(&mut game.state, &mut game.renderer);
        self.game = Some(game);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match (event, &mut self.game) {
            (WindowEvent::CloseRequested, _) => event_loop.exit(),
            (WindowEvent::Resized(size), Some(game)) => game.window.resize(size),
            (WindowEvent::KeyboardInput { device_id:_ , event, is_synthetic:_ }, Some(game)) => {
                match event.physical_key {
                    PhysicalKey::Code(key) => input::keyboard_input(key, &mut game.state.game_state),
                    _ => {}
                }
            },
            (WindowEvent::CursorMoved { device_id:_, position }, Some(game)) => {
                game.state.input_state.user_state.mouse_position = position;
            },
            (WindowEvent::MouseInput { device_id:_, state, button }, Some(game)) => {
                input::mouse_input(state, button, &mut game.state.input_state, &mut game.state.game_state);
            },
            (WindowEvent::RedrawRequested, Some(game)) => {
                game.window.render(|| game.renderer.draw_scene(&game.state.render_state));
            },
            _ => {},
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let Some(game) = &mut self.game else { return; };

        if let Some(dt) = tick(&game.state.start_time, &mut game.state.previous_tick, &Duration::from_millis(50)) {
            game.state.update(&mut game.renderer, dt);
        }

        if let Some(dt) = tick(&game.state.start_time, &mut game.state.render_state.previous_frame, &Duration::from_secs_f32(1./60.)) {
            game.state.render_state.delta_time = dt;
            game.window.request_redraw();
        }
    }
}

fn tick(start:&Instant, previous:&mut Duration, length:&Duration) -> Option<f64> {
    let current = start.elapsed();
    let delta_time = current - *previous;
    if delta_time >= *length {
        *previous = current;
        Some(delta_time.as_secs_f64())
    } else { None }
}

