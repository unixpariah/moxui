use std::sync::Arc;

use gui_lib::tree;
use winit::window::Window;

pub struct WgpuCtx<'window> {
    pub surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub tree: tree::Tree,
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

        let tree = tree::Tree::new(&device, &surface_config, |surface| {
            surface
                .set_background_color(0.0, 0.0, 0.0, 0.0)
                .add_child(|item| {
                    item.set_background_color(0.0, 0.0, 1.0, 1.0)
                        .set_size(100.0, 100.0)
                        .set_coordinates(100.0, 700.0)
                        .set_border_radius(0.0, 10.0, 30.0, 50.0)
                        .set_border_color(1.0, 1.0, 1.0, 1.0)
                        .set_border_size(2.0, 2.0, 2.0, 2.0)
                        .add_child(|item| {
                            item.set_background_color(0.0, 1.0, 0.0, 1.0)
                                .set_size(300.0, 10.0)
                                .set_coordinates(600.0, 300.0)
                                .set_border_radius(10.0, 10.0, 10.0, 10.0)
                        })
                })
                .add_child(|item| {
                    item.set_background_color(1.0, 0.0, 0.0, 1.0)
                        .set_size(300.0, 300.0)
                        .set_coordinates(200.0, 100.0)
                        .set_border_radius(10.0, 10.0, 10.0, 10.0)
                })
                .add_child(|item| {
                    item.set_background_color(0.0, 1.0, 0.0, 1.0)
                        .set_size(100.0, 100.0)
                        .set_coordinates(10.0, 100.0)
                        .set_border_radius(55.0, 55.0, 55.0, 55.0)
                        .set_boxshadow_offset(0.0, 10.0)
                        .set_boxshadow_color(1.0, 1.0, 0.0, 1.0)
                        .set_boxshadow_softness(30.0)
                })
                .add_child(|item| {
                    item.set_background_color(0.0, 1.0, 0.0, 1.0)
                        .set_size(100.0, 100.0)
                        .set_coordinates(100.0, 500.0)
                        .set_border_radius(10.0, 10.0, 10.0, 10.0)
                        .set_border_size(0.0, 5.0, 10.0, 15.0)
                        .set_border_color(1.0, 1.0, 0.0, 1.0)
                        .set_outline_width(5.0)
                        .set_outline_color(1.0, 0.0, 0.0, 1.0)
                        .set_outline_offset(50.0)
                        .set_boxshadow_offset(0.0, 10.0)
                        .set_boxshadow_color(1.0, 1.0, 0.0, 1.0)
                        .set_boxshadow_softness(30.0)
                })
                .add_child(|item| {
                    item.set_background_color(0.0, 1.0, 0.0, 1.0)
                        .set_size(100.0, 100.0)
                        .set_coordinates(100.0, 500.0)
                        .set_border_radius(10.0, 10.0, 10.0, 10.0)
                        .set_border_size(0.0, 5.0, 10.0, 15.0)
                        .set_border_color(1.0, 1.0, 0.0, 1.0)
                        .set_outline_width(5.0)
                        .set_outline_color(1.0, 0.0, 0.0, 1.0)
                        .set_outline_offset(50.0)
                        .set_boxshadow_offset(0.0, 10.0)
                        .set_boxshadow_color(1.0, 1.0, 0.0, 1.0)
                        .set_boxshadow_softness(30.0)
                })
        });

        WgpuCtx {
            tree,
            surface,
            surface_config,
            adapter,
            device,
            queue,
        }
    }
}
