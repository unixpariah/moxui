use crate::{buffers, rectangle};
use std::ops::{Deref, DerefMut};

pub struct Tree {
    projection: buffers::ProjectionUniform,
    node: Node,
}

impl Tree {
    pub fn new(device: &wgpu::Device, rectangle: rectangle::Rectangle) -> Self {
        Self {
            projection: buffers::ProjectionUniform::new(&device, 0.0, 1920.0, 0.0, 1080.0),
            node: Node::new(rectangle),
        }
    }

    pub fn render(&self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass) {
        let indices: &[u16] = &[0, 1, 3, 1, 2, 3];
        let index_buffer = buffers::IndexBuffer::new(&device, indices);

        let rect_buf = buffers::VertexBuffer::new(
            &device,
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
        );

        render_pass.set_bind_group(0, &self.projection.bind_group, &[]);
        render_pass.set_vertex_buffer(0, rect_buf.slice(..));

        let mut instances = Vec::new();
        self.collect_instances(&mut instances);

        let instance_buffer = buffers::InstanceBuffer::new(device, &instances);
        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));

        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..index_buffer.size(), 0, 0..instance_buffer.size());
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
