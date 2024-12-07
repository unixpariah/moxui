use std::sync::Arc;

use moxui::{rectangle::Units, tree};
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
                    let mut item = item
                        .set_background_color(0.0, 0.0, 1.0, 1.0)
                        //.set_size(Units::Px(100.0), Units::Px(100.0))
                        .set_border_radius(0.0, 10.0, 30.0, 50.0)
                        .set_border_color(1.0, 1.0, 1.0, 1.0)
                        .set_border_size(5.0, 5.0, 5.0, 5.0)
                        .set_margin(
                            Units::Px(0.0),
                            Units::Px(0.0),
                            Units::Px(50.0),
                            Units::Px(0.0),
                        );
                    item.style.height = Some(Units::Px(100.0));
                    item
                })
                .add_child(|item| {
                    item.set_background_color(1.0, 0.0, 0.0, 1.0)
                        .set_size(Units::Px(300.0), Units::Px(300.0))
                        .set_border_radius(10.0, 10.0, 10.0, 10.0)
                        .set_margin(
                            Units::Px(0.0),
                            Units::Px(0.0),
                            Units::Px(0.0),
                            Units::Px(20.0),
                        )
                        .add_child(|item| {
                            item.set_background_color(0.0, 1.0, 0.0, 1.0)
                                .set_size(Units::Px(150.0), Units::Px(150.0))
                                .set_border_radius(10.0, 10.0, 10.0, 10.0)
                                .set_margin(
                                    Units::Px(40.0),
                                    Units::Px(0.0),
                                    Units::Px(0.0),
                                    Units::Px(20.0),
                                )
                                .add_child(|item| {
                                    item.set_background_color(0.0, 0.0, 1.0, 1.0)
                                        .set_size(Units::Px(50.0), Units::Px(50.0))
                                        .set_border_radius(10.0, 10.0, 10.0, 10.0)
                                        .set_margin(
                                            Units::Px(40.0),
                                            Units::Px(0.0),
                                            Units::Px(0.0),
                                            Units::Px(20.0),
                                        )
                                })
                        })
                })
                .add_child(|item| {
                    item.set_background_color(0.0, 1.0, 0.0, 1.0)
                        .set_size(Units::Px(100.0), Units::Px(100.0))
                        .set_border_radius(55.0, 55.0, 55.0, 55.0)
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
