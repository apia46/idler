use vello::{AaConfig, Renderer, RendererOptions, Scene, kurbo::{Affine, Circle}, peniko::Color, util::{RenderContext, RenderSurface}};
use wgpu::{CurrentSurfaceTexture, TextureViewDescriptor, wgt::CommandEncoderDescriptor};
use winit::{application::ApplicationHandler, dpi::PhysicalSize, error::EventLoopError, event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowId}};
use std::sync::Arc;
use futures::executor::block_on;

pub mod num;
pub mod num2;

#[derive(Default)]
struct AppHandler {
    app: Option<App>
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.app = Some(block_on(App::new(event_loop)));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match (event, &mut self.app) {
            (WindowEvent::CloseRequested, _) => event_loop.exit(),
            (WindowEvent::Resized(size), Some(app)) => app.resize(size),
            (WindowEvent::KeyboardInput { device_id:_ , event, is_synthetic:_ }, _) => {
                println!("{event:?}");
            },
            (WindowEvent::RedrawRequested, Some(app)) => app.render(),
            _ => {},
        }
    }
}

struct App {
    window: Arc<Window>,
    render_context:RenderContext,
    surface: RenderSurface<'static>,
    renderers: Vec<Option<Renderer>>,
    valid_surface: bool,
    scene: Scene,
}

impl App {
    async fn new(event_loop:&ActiveEventLoop) -> Self {
        let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
        let size = window.inner_size();
        let mut render_context = RenderContext::new();
        // this also creates the device
        let surface = render_context.create_surface(window.clone(), size.width, size.height, wgpu::PresentMode::AutoVsync).await.unwrap();
        let mut renderers = vec![];
        renderers.resize_with(render_context.devices.len(), || None);
        renderers[surface.dev_id].get_or_insert_with(||
            Renderer::new(&render_context.devices[surface.dev_id].device, RendererOptions::default()).unwrap()
        );
        let scene = Scene::new();


        Self {
            window, render_context, surface, renderers, valid_surface: true, scene
        }
    }

    fn resize(&mut self, size:PhysicalSize<u32>) {
        if size.width == 0 && size.height == 0 {
            self.valid_surface = false;
        } else {
            self.render_context.resize_surface(&mut self.surface, size.width, size.height);
            self.valid_surface = true;
        }
    }

    fn render(&mut self) {
        if !self.valid_surface { return; }
        self.scene.reset();

        draw(&mut self.scene);

        let device_handle = &self.render_context.devices[self.surface.dev_id];
        self.renderers[self.surface.dev_id]
            .as_mut()
            .unwrap()
            .render_to_texture(
                &device_handle.device,
                &device_handle.queue,
                &self.scene,
                &self.surface.target_view,
                &vello::RenderParams {
                    base_color: vello::peniko::Color::new([0.1, 0.2, 0.3, 1.0]),
                    width: self.surface.config.width,
                    height: self.surface.config.height,
                    antialiasing_method: AaConfig::Msaa16,
                }
            ).unwrap();

        let surface_texture = match self.surface.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            CurrentSurfaceTexture::Outdated | CurrentSurfaceTexture::Suboptimal(_) => {
                self.render_context.configure_surface(&self.surface);
                self.window.request_redraw();
                return;
            },
            CurrentSurfaceTexture::Occluded | CurrentSurfaceTexture::Timeout => {
                self.window.request_redraw();
                return;
            },
            CurrentSurfaceTexture::Lost => panic!("Surface was lost"),
            CurrentSurfaceTexture::Validation => panic!("Surface validation error"),
        };
        let view = surface_texture.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = device_handle.device.create_command_encoder(&CommandEncoderDescriptor::default());
        self.surface.blitter.copy(&device_handle.device, &mut encoder, &self.surface.target_view, &view);
        device_handle.queue.submit([encoder.finish()]);
        device_handle.queue.present(surface_texture);
        device_handle.device.poll(wgpu::PollType::Poll).unwrap();
    }
}

fn draw(scene:&mut Scene) {
    let circle = Circle::new((420.0, 200.0), 120.0);
    let circle_fill_color = Color::new([0.9529, 0.5451, 0.6588, 1.]);
    scene.fill(
        vello::peniko::Fill::NonZero,
        Affine::IDENTITY,
        circle_fill_color,
        None,
        &circle,
    );
}

fn main() -> Result<(), EventLoopError> {
    env_logger::init();
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let mut app = AppHandler::default();
    event_loop.run_app(&mut app)
}

