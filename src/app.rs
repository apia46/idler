use std::time::{Duration, Instant};

use vello::kurbo::Vec2;
use winit::{application::ApplicationHandler, error::EventLoopError, event::{MouseScrollDelta, WindowEvent}, event_loop::{ActiveEventLoop, EventLoop}, window::WindowId};

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
            (WindowEvent::Focused(true), Some(game)) if let Some(refresh_rate) = game.window.get_refresh_rate() => {
                // i cant figure out a better place to put this
                game.state.render_state.renderer_state.refresh_rate = refresh_rate;
            },
            (WindowEvent::CloseRequested, _) => event_loop.exit(),
            (WindowEvent::Resized(size), Some(game)) => game.window.resize(size),
            (WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer:_ }, Some(game)) => {
                game.window.update_scale_factor(scale_factor);
            },
            (WindowEvent::KeyboardInput { device_id:_ , event, is_synthetic:_ }, Some(game)) => {
                input::keyboard_input(event, &mut game.state);
            },
            (WindowEvent::CursorMoved { device_id:_, position }, Some(game)) => {
                input::mouse_moved(game.window.to_screen(position), &mut game.state);
            },
            (WindowEvent::MouseWheel { device_id:_, delta, phase:_ }, Some(game)) => {
                input::mouse_scroll(match delta {
                    MouseScrollDelta::LineDelta(x, y) => Vec2::new(x as f64, y as f64) * 16.,
                    MouseScrollDelta::PixelDelta(position) => Vec2::new(position.x, position.y),
                }, &mut game.state);
            },
            (WindowEvent::MouseInput { device_id:_, state, button }, Some(game)) => {
                input::mouse_input(state, button, &mut game.state);
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

        let renderer_state = &mut game.state.render_state.renderer_state;
        if let Some(dt) = tick(&game.state.start_time, &mut renderer_state.previous_frame, &renderer_state.refresh_rate) {
            game.state.render_state.renderer_state.delta_time = dt;
            game.state.process(&mut game.renderer, dt);
            game.window.request_redraw();
        }
    }
}

fn tick(start:&Instant, previous:&mut Duration, length:&Duration) -> Option<Duration> {
    let current = start.elapsed();
    let delta_time = current - *previous;
    if delta_time >= *length {
        *previous = current;
        Some(delta_time)
    } else { None }
}

