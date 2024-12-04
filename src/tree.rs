use crate::{buffers, rectangle};
use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct Tree {
    projection_uniform: buffers::ProjectionUniform,
    index_buffer: buffers::IndexBuffer,
    generic_rect: buffers::VertexBuffer,
    node: Node,
}

impl Tree {
    pub fn new(device: &wgpu::Device, surface: rectangle::Rectangle) -> Self {
        let extents = surface.get_extents();
        let projection_uniform = buffers::ProjectionUniform::new(
            device,
            extents.x,
            extents.x + extents.width,
            extents.y,
            extents.y + extents.height,
        );

        Self {
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
            node: Node::new(surface),
        }
    }

    pub fn bind_group_layouts(&self) -> Rc<[&wgpu::BindGroupLayout]> {
        Rc::new([&self.projection_uniform.bind_group_layout])
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

impl Node {
    pub fn new(rectangle: rectangle::Rectangle) -> Self {
        return Self {
            data: rectangle,
            children: Vec::new(),
        };
    }

    pub fn add_child(&mut self, rectangle: rectangle::Rectangle) {
        let node = Node {
            data: rectangle,
            children: Vec::new(),
        };

        self.children.push(node);
    }

    fn collect_instances(&self, instances: &mut Vec<buffers::Instance>) {
        instances.push(self.data.get_instance());

        self.children
            .iter()
            .for_each(|child| child.collect_instances(instances));
    }
}
