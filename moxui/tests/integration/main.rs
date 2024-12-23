mod css2;

use moxui::tree;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    error::EventLoopError,
    event::{MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowId},
};

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app)
}

#[derive(Default)]
pub struct App<'window> {
    wgpu_ctx: Option<WgpuCtx<'window>>,
    window: Option<Arc<Window>>,
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let win_attr = Window::default_attributes().with_title("wgpu winit example");
            let window = Arc::new(
                event_loop
                    .create_window(win_attr)
                    .expect("create window err."),
            );
            self.window = Some(window.clone());
            let mut wgpu_ctx = WgpuCtx::new(window.clone());
            wgpu_ctx.trees.extend(css2::css2(&wgpu_ctx));

            self.wgpu_ctx = Some(wgpu_ctx);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(wgpu_ctx) = &mut self.wgpu_ctx {
                    wgpu_ctx.draw();
                }
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if !event.state.is_pressed() {
                    return;
                }

                let Key::Named(key) = event.logical_key else {
                    return;
                };

                let Some(mut wgpu_ctx) = self.wgpu_ctx.take() else {
                    return;
                };

                match key {
                    NamedKey::ArrowLeft => {
                        if wgpu_ctx.index > 0 {
                            wgpu_ctx.index -= 1;
                        }
                    }
                    NamedKey::ArrowRight => {
                        if wgpu_ctx.index == wgpu_ctx.trees.len() - 1 {
                            event_loop.exit();
                            return;
                        }
                        wgpu_ctx.index += 1;
                    }
                    _ => {}
                }

                wgpu_ctx.draw();
                self.wgpu_ctx = Some(wgpu_ctx);
            }
            WindowEvent::MouseWheel {
                device_id: _,
                delta,
                phase: _,
            } => {
                let Some(ref mut wgpu_ctx) = self.wgpu_ctx else {
                    return;
                };

                if let MouseScrollDelta::PixelDelta(PhysicalPosition { x, y }) = delta {
                    let tree = &mut wgpu_ctx.trees[wgpu_ctx.index];
                    tree.scroll(&wgpu_ctx.device, x as f32, y as f32);
                    wgpu_ctx.draw();
                }
            }
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                let Some(ref mut wgpu_ctx) = self.wgpu_ctx else {
                    return;
                };

                let tree = &mut wgpu_ctx.trees[wgpu_ctx.index];
                tree.set_viewport(&wgpu_ctx.device, width as f32, height as f32);
                wgpu_ctx.draw();
            }
            _ => (),
        }
    }
}

#[allow(dead_code)]
pub struct WgpuCtx<'window> {
    pub index: usize,
    pub surface: wgpu::Surface<'window>,
    pub surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub trees: Vec<tree::Tree>,
}

impl<'window> WgpuCtx<'window> {
    pub fn new(window: Arc<Window>) -> WgpuCtx<'window> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .expect("Failed to find suitable adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(&Default::default(), None))
            .expect("Failed to request device");

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);
        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
        surface.configure(&device, &surface_config);

        WgpuCtx {
            index: 0,
            trees: Vec::new(),
            surface,
            surface_config,
            adapter,
            device,
            queue,
        }
    }

    pub fn draw(&mut self) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.trees[self.index].render(&self.device, &self.queue, &mut rpass);

        drop(rpass);

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}
