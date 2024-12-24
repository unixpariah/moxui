use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::RwLock,
};

use crate::rectangle::{self, InstanceData};
use calc_units::{Context, Units};
use glyphon::{Attrs, Buffer, Family, FontSystem};

use super::State;

pub struct Text {
    pub buffer: Buffer,
    pub font_system: FontSystem,
}

pub struct Node {
    pub(crate) children: Vec<Node>,
    pub(crate) data: rectangle::Rectangle,
    pub(crate) text: Option<Text>,
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
            text: None,
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
        let x = self.x;
        let y = self.y;
        let width = self.width;
        let height = self.height;

        let mut children = collect_children(&mut self.children);
        children.iter_mut().for_each(|child| {
            let hor_context = &Context {
                parent_size: width,
                viewport: state.viewport,
                dpi: state.dpi,
                auto: 0.0,
            };
            let vert_context = &Context {
                parent_size: height,
                viewport: state.viewport,
                dpi: state.dpi,
                auto: 0.0,
            };

            match child.style.position {
                rectangle::Position::Static | rectangle::Position::Sticky => {}

                rectangle::Position::Relative => {
                    child.x += child.style.top.to_px(&hor_context)
                        - child.style.bottom.to_px(&hor_context);
                    child.y += child.style.left.to_px(&vert_context)
                        - child.style.right.to_px(&vert_context);
                }
                rectangle::Position::Fixed => {
                    child.x = x + child.style.left.to_px(&hor_context);
                    child.y = y + child.style.top.to_px(&hor_context);
                }
                rectangle::Position::Absolute => {
                    child.x = x + child.style.left.to_px(&hor_context);
                    child.y = y + child.style.top.to_px(&vert_context);
                }
            }

            let hor_context = Context {
                parent_size: width,
                viewport: state.viewport,
                dpi: state.dpi,
                auto: 0.0,
            };

            child.outline.width = child.style.outline_width.to_px(&hor_context);
            child.outline.offset = child.style.outline_offset.to_px(&hor_context);

            (0..4).for_each(|i| {
                child.padding[i] = child.style.padding[i].to_px(&hor_context);
                child.margin[i] = child.style.margin[i].to_px(&hor_context);
                child.border.size[i] = child.style.border[i].to_px(&hor_context);
            });

            match child.style.display {
                rectangle::Display::Block => {
                    //(child.x, child.y) = (0.0, current_pos.1.max(total_size.1));

                    child.width = child.style.width.to_px(&Context {
                        parent_size: width,
                        viewport: state.viewport,
                        dpi: state.dpi,
                        auto: width,
                    });
                    let s = child.position_children();
                    child.height = child.style.height.to_px(&Context {
                        parent_size: height,
                        viewport: state.viewport,
                        dpi: state.dpi,
                        auto: s.1,
                    });

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

                    //(child.x, child.y) = current_pos;

                    current_pos.0 += child_extents.width;
                    total_size.0 = current_pos.0.max(total_size.0);
                    total_size.1 = (current_pos.1 + child_extents.height).max(total_size.1);
                }
                rectangle::Display::InlineBlock => {
                    let s = child.position_children();
                    child.width = child.style.width.to_px(&Context {
                        parent_size: width,
                        viewport: state.viewport,
                        dpi: state.dpi,
                        auto: s.0,
                    });
                    child.height = child.style.height.to_px(&Context {
                        parent_size: height,
                        viewport: state.viewport,
                        dpi: state.dpi,
                        auto: s.1,
                    });

                    let child_extents = child.get_extents();

                    if current_pos.0 + child_extents.width > width && child_extents.width < width {
                        current_pos.0 = 0.0;
                        current_pos.1 = total_size.1;
                    }

                    //(child.x, child.y) = current_pos;

                    current_pos.0 += child_extents.width;
                    total_size.0 = current_pos.0.max(total_size.0);
                    total_size.1 = (current_pos.1 + child_extents.height).max(total_size.1);
                }
                _ => {}
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

    pub(crate) fn collect_instances(
        &self,
        instance_data: &mut Vec<InstanceData>,
        buffers: &mut Vec<TextData>,
    ) {
        if self.style.display == rectangle::Display::None {
            return;
        }

        if self.style.display != rectangle::Display::Contents {
            instance_data.push(self.data.get_instance_data());
            if let Some(text) = &self.text {
                buffers.push(TextData {
                    x: self.data.x,
                    y: self.data.y,
                    buffer: text.buffer.clone(),
                });
            }
        }

        self.children
            .iter()
            .for_each(|child| child.collect_instances(instance_data, buffers));
    }
}

pub struct TextData {
    pub x: f32,
    pub y: f32,
    pub buffer: Buffer,
}

impl Node {
    pub fn set_content(mut self, content: &str) -> Self {
        if self.text.is_none() {
            let mut font_system = FontSystem::new();

            self.text = Some(Text {
                buffer: glyphon::Buffer::new(&mut font_system, glyphon::Metrics::new(30.0, 42.0)),
                font_system,
            })
        }

        let text = self.text.as_mut().unwrap();
        text.buffer.set_text(
            &mut text.font_system,
            content,
            Attrs::new().family(Family::SansSerif),
            glyphon::Shaping::Advanced,
        );

        self
    }

    pub fn set_coordinates(mut self, top: Units, right: Units, bottom: Units, left: Units) -> Self {
        self.style.top = top;
        self.style.right = right;
        self.style.bottom = bottom;
        self.style.left = left;
        self
    }

    pub fn set_position(mut self, position: rectangle::Position) -> Self {
        self.style.position = position;
        self
    }

    pub fn set_display(mut self, display: rectangle::Display) -> Self {
        self.style.display = display;
        self
    }

    pub fn set_size(mut self, width: Units, height: Units) -> Self {
        self.style.width = width;
        self.style.height = height;
        self
    }

    pub fn set_box_sizing(mut self, box_sizing: rectangle::BoxSizing) -> Self {
        self.style.box_sizing = box_sizing;
        self
    }

    pub fn set_padding(mut self, top: Units, right: Units, bottom: Units, left: Units) -> Self {
        self.style.padding = [top, right, bottom, left];

        self
    }

    pub fn set_background_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.background_color = [r, g, b, a];
        self
    }

    pub fn set_margin(mut self, top: Units, right: Units, bottom: Units, left: Units) -> Self {
        self.style.margin = [top, right, bottom, left];
        self
    }

    pub fn set_border_size(mut self, top: Units, right: Units, bottom: Units, left: Units) -> Self {
        self.style.border = [top, right, bottom, left];
        self
    }

    pub fn set_border_top_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.border.top_color = [r, g, b, a];
        self
    }

    pub fn set_border_bottom_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.border.bottom_color = [r, g, b, a];
        self
    }

    pub fn set_border_left_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.border.left_color = [r, g, b, a];
        self
    }

    pub fn set_border_right_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.border.right_color = [r, g, b, a];
        self
    }

    pub fn set_border_radius(
        mut self,
        top_left: f32,
        top_right: f32,
        bottom_right: f32,
        bottom_left: f32,
    ) -> Self {
        self.border.radius = [top_left, top_right, bottom_right, bottom_left];
        self
    }

    pub fn set_outline_width(mut self, width: Units) -> Self {
        self.style.outline_width = width;
        self
    }

    pub fn set_outline_offset(mut self, offset: Units) -> Self {
        self.style.outline_offset = offset;
        self
    }

    pub fn set_outline_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.outline.color = [r, g, b, a];
        self
    }

    pub fn set_sepia(mut self, sepia: f32) -> Self {
        self.sepia = sepia;
        self
    }

    pub fn set_opacity(mut self, opacity: f32) -> Self {
        self.background_color[3] = opacity;
        self
    }

    pub fn set_brightness(mut self, brightness: f32) -> Self {
        self.brightness = brightness;
        self
    }

    pub fn set_contrast(mut self, contrast: f32) -> Self {
        self.contrast = contrast;
        self
    }

    pub fn set_grayscale(mut self, grayscale: f32) -> Self {
        self.grayscale = grayscale;
        self
    }

    pub fn set_hue_rotate(mut self, hue_rotate: f32) -> Self {
        self.hue_rotate = hue_rotate;
        self
    }

    pub fn set_invert(mut self, invert: f32) -> Self {
        self.invert = invert;
        self
    }

    pub fn set_saturate(mut self, saturate: f32) -> Self {
        self.saturate = saturate;
        self
    }

    pub fn set_scale(mut self, x: f32, y: f32) -> Self {
        self.scale = [x, y];
        self
    }

    pub fn set_skew(mut self, x: f32, y: f32) -> Self {
        self.skew = [x, y];
        self
    }

    pub fn set_rotate(mut self, rotation: f32) -> Self {
        self.rotate = rotation;
        self
    }

    pub fn set_translate(mut self, translate: [f32; 2]) -> Self {
        self.translate = translate;
        self
    }
}
