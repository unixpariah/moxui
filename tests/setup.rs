use std::sync::Arc;

use gui_lib::tree;
use winit::window::Window;

pub struct WgpuCtx<'window> {
    pub surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub render_pipeline: wgpu::RenderPipeline,
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

        let tree = tree::Tree::new(&device, |surface| {
            surface
                .set_size(surface_config.width as f32, surface_config.height as f32)
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

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &tree.bind_group_layouts(),
                push_constant_ranges: &[],
            });

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .unwrap_or(&surface_caps.formats[0]);

        let alpha_mode = surface_caps
            .alpha_modes
            .iter()
            .find(|a| **a == wgpu::CompositeAlphaMode::PreMultiplied)
            .unwrap_or(&surface_caps.alpha_modes[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *surface_format,
            width: 1,
            height: 1,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: *alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: tree.shader_module(),
                entry_point: Some("vs_main"),
                buffers: &tree::Tree::buffer_layouts(),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: tree.shader_module(),
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            depth_stencil: None,
            multiview: None,
            cache: None,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        WgpuCtx {
            tree,
            surface,
            surface_config,
            adapter,
            device,
            queue,
            render_pipeline,
        }
    }
}
