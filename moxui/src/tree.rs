mod node;
mod text;

use crate::buffers;
use glyphon::{Color, TextArea, TextBounds};
use node::Node;
use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::RwLock,
};
use text::Text;

pub struct Tree {
    pub render_pipeline: wgpu::RenderPipeline,
    pub projection_uniform: buffers::ProjectionUniform,
    pub index_buffer: buffers::IndexBuffer,
    pub generic_rect: buffers::VertexBuffer,
    pub node: node::Node,
    pub text: text::Text,
}

pub struct Config {
    pub width: f32,
    pub height: f32,
    pub dpi: f32,
    pub format: wgpu::TextureFormat,
}

pub struct State {
    pub viewport: (f32, f32),
    pub scroll: (f32, f32),
    pub dpi: f32,
}

impl Tree {
    pub fn new<F>(device: &wgpu::Device, queue: &wgpu::Queue, config: &Config, f: F) -> Self
    where
        F: Fn(node::Node) -> node::Node,
    {
        let text = Text::new(device, queue, config);

        let state = State {
            viewport: (config.width as f32, config.height as f32),
            scroll: (0.0, 0.0),
            dpi: config.dpi,
        };

        let mut node = node::Node::new(Rc::new(RwLock::new(state)));
        node.width = config.width as f32;
        node.height = config.height as f32;
        let node = f(node);

        let projection_uniform = buffers::ProjectionUniform::new(
            device,
            0.0,
            config.width as f32,
            0.0,
            config.height as f32,
        );

        let storage_buffer_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &projection_uniform.bind_group_layout,
                    &storage_buffer_layout,
                ],
                push_constant_ranges: &[],
            });

        let vertex_buffers = [buffers::Vertex::desc()];

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &vertex_buffers,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
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

        let generic_rect_vertices = [
            buffers::Vertex {
                position: [0.0, 1.0],
            },
            buffers::Vertex {
                position: [1.0, 1.0],
            },
            buffers::Vertex {
                position: [1.0, 0.0],
            },
            buffers::Vertex {
                position: [0.0, 0.0],
            },
        ];

        Self {
            text,
            render_pipeline,
            index_buffer: buffers::IndexBuffer::new(device, &[0, 1, 3, 1, 2, 3]),
            generic_rect: buffers::VertexBuffer::new(device, &generic_rect_vertices),
            projection_uniform,
            node,
        }
    }

    pub fn scroll(&mut self, device: &wgpu::Device, x: f32, y: f32) {
        let state = self.state.clone();
        let mut state = state.write().unwrap();

        let new_x = state.scroll.0 + x;
        let new_y = state.scroll.1 + y;

        state.scroll.0 = new_x.clamp(0.0, self.width - state.viewport.0);
        state.scroll.1 = new_y.clamp(0.0, self.height - state.viewport.1);

        self.projection_uniform = buffers::ProjectionUniform::new(
            device,
            state.scroll.0,
            state.scroll.0 + state.viewport.0,
            state.scroll.1,
            state.scroll.1 + state.viewport.1,
        );
    }

    pub fn set_viewport(&mut self, device: &wgpu::Device, width: f32, height: f32) {
        let state = self.state.clone();
        let mut state = state.write().unwrap();

        self.projection_uniform = buffers::ProjectionUniform::new(
            device,
            state.scroll.0,
            state.scroll.0 + width,
            state.scroll.1,
            state.scroll.1 + height,
        );
        state.viewport = (width, height);
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass,
    ) {
        let mut instance_data = Vec::new();
        let mut text_data = Vec::new();

        self.collect_instances(&mut instance_data, &mut text_data);

        let text_data = text_data
            .iter()
            .map(|text_data| {
                let (width, total_lines) = text_data
                    .buffer
                    .layout_runs()
                    .fold((0.0, 0usize), |(width, total_lines), run| {
                        (run.line_w.max(width), total_lines + 1)
                    });

                TextArea {
                    buffer: &text_data.buffer,
                    top: text_data.x,
                    left: text_data.y,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: width as i32,
                        bottom: (total_lines as f32 * text_data.buffer.metrics().line_height)
                            as i32,
                    },
                    default_color: Color::rgb(255, 255, 255),
                    custom_glyphs: &[],
                }
            })
            .collect();

        let storage_buffer = buffers::StorageBuffer::new(device, instance_data.into());

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.projection_uniform.bind_group, &[]);
        render_pass.set_bind_group(1, &storage_buffer.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.generic_rect.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.index_buffer.size(), 0, 0..storage_buffer.len());

        self.text.render(device, queue, render_pass, text_data);
    }

    pub fn finish(mut self) -> Self {
        let a = self.position_children();
        self.width = self.width.max(a.0);
        self.height = self.height.max(a.1);

        self
    }
}

impl Deref for Tree {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl DerefMut for Tree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}
