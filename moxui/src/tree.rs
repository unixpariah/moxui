mod node;

use crate::{
    buffers,
    rectangle::{self, InstanceData, State},
};
use calc_units::Context;
use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::RwLock,
};

pub struct Tree {
    render_pipeline: wgpu::RenderPipeline,
    projection_uniform: buffers::ProjectionUniform,
    index_buffer: buffers::IndexBuffer,
    generic_rect: buffers::VertexBuffer,
    node: Node,
}

impl Tree {
    pub fn new<F>(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, f: F) -> Self
    where
        F: Fn(Node) -> Node,
    {
        let state = State {
            viewport: (config.width as f32, config.height as f32),
            scroll: (0.0, 0.0),
        };

        let mut node = Node::new(Rc::new(RwLock::new(state)));
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

        let vertex_buffers = [buffers::Vertex::desc(), buffers::Instance::desc()];

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

    pub fn render(&self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass) {
        let mut instance_data = Vec::new();
        let mut instances = Vec::new();
        self.collect_instances(&mut instances, &mut instance_data);
        let storage_buffer = buffers::StorageBuffer::new(device, &instance_data);
        let instance_buffer = buffers::InstanceBuffer::new(device, &instances);

        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.projection_uniform.bind_group, &[]);
        render_pass.set_bind_group(1, &storage_buffer.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.generic_rect.slice(..));
        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.index_buffer.size(), 0, 0..instance_buffer.size());
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

pub struct Node {
    pub children: Vec<Node>,
    pub data: rectangle::Rectangle,
}

impl Deref for Node {
    type Target = rectangle::Rectangle;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

// Recursively collect children with display: contents; parent and temporarily 'reparent' them
pub fn collect_children(children: &mut Vec<Node>) -> Vec<&mut Node> {
    children
        .iter_mut()
        .flat_map(|child| {
            if child.style.display == rectangle::Display::Contents {
                collect_children(&mut child.children)
            } else {
                vec![child]
            }
        })
        .collect()
}

impl Node {
    pub fn new(state: Rc<RwLock<State>>) -> Self {
        return Self {
            data: rectangle::Rectangle::new(state),
            children: Vec::new(),
        };
    }

    pub fn position_children(&mut self) -> (f32, f32) {
        let state = self.state.clone();
        let state = state.read().unwrap();

        let mut current_pos = (
            self.x + self.margin[3] + self.padding[3],
            self.y + self.margin[0] + self.padding[0],
        );

        let mut total_size = (0.0, 0.0);
        let width = self.width;

        let vert_context = Context {
            parent_size: self.height,
            viewport: state.viewport,
        };
        let hor_context = Context {
            parent_size: self.width,
            viewport: state.viewport,
        };

        let extents = self.get_extents();
        let mut children = collect_children(&mut self.children);
        children.iter_mut().for_each(|child| {
            (0..4).for_each(|i| {
                child.padding[i] = child.style.padding[i].to_px(&hor_context);
                child.margin[i] = child.style.margin[i].to_px(&hor_context);
                child.border.size[i] = child.style.border[i].to_px(&hor_context);
            });

            match child.style.display {
                rectangle::Display::Block => {
                    (child.x, child.y) = (0.0, current_pos.1.max(total_size.1));

                    child.width = match &child.style.width {
                        None => extents.width,
                        Some(units) => units.to_px(&hor_context),
                    };

                    child.height = match &child.style.height {
                        None => child.position_children().1,
                        Some(units) => units.to_px(&vert_context),
                    };

                    let child_extents = child.get_extents();
                    current_pos.0 = 0.0;
                    current_pos.1 = child.y + child_extents.height;
                    total_size.0 = child_extents.width.max(total_size.0);
                    total_size.1 += child_extents.height;
                }
                rectangle::Display::Inline => {
                    (child.width, child.height) = child.position_children();
                    child.height -=
                        child.margin[0] + child.margin[2] + child.padding[0] + child.padding[2];

                    let child_extents = child.get_extents();

                    if current_pos.0 + child_extents.width >= width {
                        current_pos.0 = 0.0;
                        current_pos.1 = total_size.1;
                    }

                    (child.x, child.y) = current_pos;

                    current_pos.0 += child_extents.width;
                    total_size.0 = current_pos.0.max(total_size.0);
                    total_size.1 = (current_pos.1 + child_extents.height).max(total_size.1);
                }
                rectangle::Display::InlineBlock => {
                    let s = child.position_children();
                    child.width = match &child.style.width {
                        None => s.0,
                        Some(units) => units.to_px(&hor_context),
                    };

                    child.height = match &child.style.height {
                        None => s.1,
                        Some(units) => units.to_px(&vert_context),
                    };

                    let child_extents = child.get_extents();

                    if current_pos.0 + child_extents.width > width && child_extents.width < width {
                        current_pos.0 = 0.0;
                        current_pos.1 = total_size.1;
                    }

                    (child.x, child.y) = current_pos;

                    current_pos.0 += child_extents.width;
                    total_size.0 = current_pos.0.max(total_size.0);
                    total_size.1 = (current_pos.1 + child_extents.height).max(total_size.1);
                }
                rectangle::Display::Flex => {}
                rectangle::Display::InlineFlex => {}
                rectangle::Display::Grid => {}
                rectangle::Display::InlineGrid => {}
                rectangle::Display::Table => {}
                rectangle::Display::InlineTable => {}
                rectangle::Display::ListItem => {}
                rectangle::Display::RunIn => {}
                rectangle::Display::Contents | rectangle::Display::None => {}
            }

            match child.style.position {
                rectangle::Position::Static => {}
                rectangle::Position::Relative => {
                    child.x += child.style.x.to_px(&hor_context);
                    child.y += child.style.y.to_px(&vert_context);
                }
                rectangle::Position::Fixed => {
                    child.x = child.style.x.to_px(&hor_context);
                    child.y = child.style.y.to_px(&hor_context);
                }
                rectangle::Position::Sticky => {}
                rectangle::Position::Absolute => {
                    child.x = child.style.x.to_px(&hor_context);
                    child.y = child.style.y.to_px(&vert_context);
                }
            }
        });

        (
            total_size.0 + self.margin[3] + self.margin[1] + self.padding[3] + self.padding[1],
            total_size.1 + self.margin[0] + self.margin[2] + self.padding[0] + self.padding[2],
        )
    }

    pub fn add_child<F>(mut self, f: F) -> Self
    where
        F: Fn(Node) -> Node,
    {
        let node = f(Node::new(self.state.clone()));
        self.children.push(node);

        self
    }

    fn collect_instances(
        &self,
        instances: &mut Vec<buffers::Instance>,
        new_instances: &mut Vec<InstanceData>,
    ) {
        if self.style.display == rectangle::Display::None {
            return;
        }

        if self.style.display != rectangle::Display::Contents {
            new_instances.push(self.data.get_instance_data());
            instances.push(self.data.get_instance());
        }

        self.children
            .iter()
            .for_each(|child| child.collect_instances(instances, new_instances));
    }
}
