mod node;

use crate::{buffers, rectangle};
use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct Tree {
    shader: wgpu::ShaderModule,
    projection_uniform: buffers::ProjectionUniform,
    index_buffer: buffers::IndexBuffer,
    generic_rect: buffers::VertexBuffer,
    node: Node,
}

impl Tree {
    pub fn new<F>(device: &wgpu::Device, f: F) -> Self
    where
        F: Fn(Node) -> Node,
    {
        let node = f(Node::new());

        let extents = node.data.get_extents();
        let projection_uniform = buffers::ProjectionUniform::new(
            device,
            extents.x,
            extents.x + extents.width,
            extents.y,
            extents.y + extents.height,
        );

        Self {
            shader: device.create_shader_module(wgpu::include_wgsl!("shader.wgsl")),
            index_buffer: buffers::IndexBuffer::new(device, &[0, 1, 3, 1, 2, 3]),
            generic_rect: buffers::VertexBuffer::new(
                device,
                &[
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
                ],
            ),
            projection_uniform,
            node,
        }
    }

    pub fn bind_group_layouts(&self) -> Rc<[&wgpu::BindGroupLayout]> {
        Rc::new([&self.projection_uniform.bind_group_layout])
    }

    pub fn buffer_layouts() -> Rc<[wgpu::VertexBufferLayout<'static>]> {
        Rc::new([buffers::Vertex::desc(), buffers::Instance::desc()])
    }

    pub fn shader_module(&self) -> &wgpu::ShaderModule {
        &self.shader
    }

    pub fn render(&self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(0, &self.projection_uniform.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.generic_rect.slice(..));

        let mut instances = Vec::new();
        self.collect_instances(&mut instances);

        let instance_buffer = buffers::InstanceBuffer::new(device, &instances);
        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));

        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.index_buffer.size(), 0, 0..instance_buffer.size());
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

impl Node {
    pub fn new() -> Self {
        return Self {
            data: rectangle::Rectangle::default(),
            children: Vec::new(),
        };
    }

    pub fn add_child<F>(mut self, f: F) -> Self
    where
        F: Fn(Node) -> Node,
    {
        let node = f(Node::new());
        self.children.push(node);
        self
    }

    fn collect_instances(&self, instances: &mut Vec<buffers::Instance>) {
        instances.push(self.data.get_instance());

        self.children
            .iter()
            .for_each(|child| child.collect_instances(instances));
    }
}
