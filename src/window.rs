use vello::{AaConfig, Renderer, RendererOptions, Scene, util::{RenderContext, RenderSurface}};
use wgpu::{CurrentSurfaceTexture, TextureViewDescriptor, wgt::CommandEncoderDescriptor};
use winit::{dpi::PhysicalSize, event_loop::ActiveEventLoop};
use winit::window::Window as WinitWindow;
use std::sync::Arc;

pub struct Window {
    window: Arc<WinitWindow>,
    render_context:RenderContext,
    surface: RenderSurface<'static>,
    renderer: Renderer,
    valid_surface: bool,
}

impl Window {
    pub async fn new(event_loop:&ActiveEventLoop) -> Self {
        let window = Arc::new(event_loop.create_window(WinitWindow::default_attributes()).unwrap());
        let size = window.inner_size();
        let mut render_context = RenderContext::new();
        // this also creates the device
        let surface = render_context.create_surface(window.clone(), size.width, size.height, wgpu::PresentMode::AutoVsync).await.unwrap();
        let renderer = Renderer::new(&render_context.devices[surface.dev_id].device, RendererOptions::default()).unwrap();

        Self { window, render_context, surface, renderer, valid_surface: true }
    }

    pub fn resize(&mut self, size:PhysicalSize<u32>) {
        if size.width == 0 && size.height == 0 {
            self.valid_surface = false;
        } else {
            self.render_context.resize_surface(&mut self.surface, size.width, size.height);
            self.valid_surface = true;
        }
    }

    pub fn render(&mut self, scene: &Scene) {
        if !self.valid_surface { return; }

        let device_handle = &self.render_context.devices[self.surface.dev_id];
        self.renderer
            .render_to_texture(
                &device_handle.device,
                &device_handle.queue,
                scene,
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

