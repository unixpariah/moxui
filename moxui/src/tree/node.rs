use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{RwLock, RwLockReadGuard},
};

use crate::rectangle::{self, Display, InstanceData, Position};
use calc_units::{Context, Units};
use glyphon::{Attrs, Color, FamilyOwned, FontSystem};

use super::{
    text::{Text, TextData},
    State,
};

struct ParentState {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    font_size: f32,
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
pub fn collect_children(children: &mut [Node]) -> Vec<&mut Node> {
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
        Self {
            data: rectangle::Rectangle::new(state),
            children: Vec::new(),
            text: None,
        }
    }

    fn get_state(&self) -> ParentState {
        let extents = self.get_extents();

        ParentState {
            x: self.x,
            y: self.y,
            width: extents.width,
            height: extents.height,
            font_size: self.font_size,
        }
    }

    fn update_layout_properties(
        &mut self,
        parent_state: &ParentState,
        state: &RwLockReadGuard<'_, State>,
    ) {
        let hor_context = Context {
            root_font_size: state.root_font_size,
            parent_size: parent_state.width,
            viewport: state.viewport,
            dpi: state.dpi,
            parent_font_size: parent_state.font_size,
            auto: 0.0,
        };

        self.outline.width = self.style.outline_width.to_px(&hor_context);
        self.outline.offset = self.style.outline_offset.to_px(&hor_context);

        (0..4).for_each(|i| {
            self.padding[i] = self.style.padding[i].to_px(&hor_context);
            self.margin[i] = self.style.margin[i].to_px(&hor_context);
            self.border.size[i] = self.style.border[i].to_px(&hor_context);
        });

        self.font_size = self.style.font_size.to_px(&Context {
            root_font_size: state.root_font_size,
            parent_size: parent_state.font_size,
            parent_font_size: parent_state.font_size,
            viewport: state.viewport,
            dpi: state.dpi,
            auto: 0.0,
        });

        self.line_height = self.style.line_height.to_px(&Context {
            root_font_size: state.root_font_size,
            parent_size: self.font_size,
            parent_font_size: parent_state.font_size,
            viewport: state.viewport,
            dpi: state.dpi,
            auto: 0.0,
        });

        let font_size = self.font_size;
        let line_height = self.line_height;
        if let Some(text) = &mut self.text {
            text.buffer.set_metrics(
                &mut text.font_system,
                glyphon::Metrics::new(font_size, line_height),
            );
        }
    }

    fn update_position(
        &mut self,
        parent_state: &ParentState,
        state: &RwLockReadGuard<'_, State>,
        current_pos: (f32, f32),
    ) {
        let context = Context {
            root_font_size: state.root_font_size,
            parent_size: parent_state.width,
            parent_font_size: parent_state.font_size,
            viewport: state.viewport,
            dpi: state.dpi,
            auto: 0.0,
        };

        (self.x, self.y) = match (self.style.position, self.style.display) {
            (_, Display::None | Display::Contents) => return,
            (Position::Static | Position::Sticky | Position::Relative, Display::Block) => {
                (0.0, current_pos.1 - self.height)
            }
            (
                Position::Static | Position::Sticky | Position::Relative,
                Display::Inline | Display::InlineBlock,
            ) => {
                if self.x + self.get_extents().width > parent_state.x + parent_state.width {
                    (0.0, current_pos.1 - self.height)
                } else {
                    (current_pos.0 - self.width, current_pos.1 - self.height)
                }
            }
            (Position::Fixed | Position::Absolute, _) => (
                parent_state.x + self.style.left.to_px(&context),
                parent_state.y + self.style.top.to_px(&context),
            ),
        }
    }

    fn update_size(
        &mut self,
        parent_state: &ParentState,
        state: &RwLockReadGuard<'_, State>,
        current_pos: &mut (f32, f32),
        total_size: &mut (f32, f32),
    ) {
        let (width, height) = match &self.text {
            None => (0.0, 0.0),
            Some(text) => text.extents(),
        };

        match self.style.display {
            rectangle::Display::Block => {
                self.width = self.style.width.to_px(&Context {
                    root_font_size: state.root_font_size,
                    parent_size: parent_state.width,
                    parent_font_size: parent_state.font_size,
                    viewport: state.viewport,
                    dpi: state.dpi,
                    auto: parent_state.width,
                });
                let auto = self.position_children().1.max(height);
                self.height = self.style.height.to_px(&Context {
                    root_font_size: state.root_font_size,
                    parent_size: parent_state.height,
                    parent_font_size: parent_state.font_size,
                    viewport: state.viewport,
                    dpi: state.dpi,
                    auto,
                });

                let self_extents = self.get_extents();
                current_pos.0 = 0.0;
                current_pos.1 = self.y + self_extents.height;
                total_size.0 = self_extents.width.max(total_size.0);
                total_size.1 += self_extents.height;
            }
            rectangle::Display::Inline => {
                (self.width, self.height) = self.position_children();
                self.height -= self.margin[0] + self.margin[2] + self.padding[0] + self.padding[2];

                let self_extents = self.get_extents();

                current_pos.0 += self_extents.width;
                total_size.0 = current_pos.0.max(total_size.0);
                total_size.1 = (current_pos.1 + self_extents.height).max(total_size.1);
            }
            rectangle::Display::InlineBlock => {
                let auto = self.position_children();
                self.width = self.style.width.to_px(&Context {
                    root_font_size: state.root_font_size,
                    parent_size: parent_state.width,
                    parent_font_size: parent_state.font_size,
                    viewport: state.viewport,
                    dpi: state.dpi,
                    auto: auto.0.max(width),
                });
                self.height = self.style.height.to_px(&Context {
                    root_font_size: state.root_font_size,
                    parent_size: parent_state.height,
                    parent_font_size: parent_state.font_size,
                    viewport: state.viewport,
                    dpi: state.dpi,
                    auto: auto.1.max(height),
                });

                let self_extents = self.get_extents();

                current_pos.0 += self_extents.width;
                total_size.0 = current_pos.0.max(total_size.0);
                total_size.1 = (current_pos.1 + self_extents.height).max(total_size.1);
            }
            _ => {}
        }
    }

    fn offset_children(&mut self) {
        let x = self.x;
        let y = self.y;

        self.children.iter_mut().for_each(|child| {
            child.x += x;
            child.y += y;
            child.offset_children();
        });
    }

    pub fn position_children(&mut self) -> (f32, f32) {
        let state = self.state.clone();
        let state = state.read().unwrap();

        let mut current_pos = (
            self.margin[3] + self.padding[3],
            self.margin[0] + self.padding[0],
        );
        let mut total_size = (0.0, 0.0);

        let parent_state = self.get_state();

        let mut children = collect_children(&mut self.children);
        children.iter_mut().for_each(|child| {
            child.update_layout_properties(&parent_state, &state);
            child.update_size(&parent_state, &state, &mut current_pos, &mut total_size);
            child.update_position(&parent_state, &state, current_pos);
        });

        self.offset_children();

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
        text_data: &mut Vec<TextData>,
    ) {
        if self.style.display == rectangle::Display::None {
            return;
        }

        if self.style.display != rectangle::Display::Contents {
            instance_data.push(self.data.get_instance_data());
            if let Some(text) = &self.text {
                let (width, height) = text.extents();

                text_data.push(TextData {
                    x: self.data.x,
                    y: self.data.y,
                    width,
                    height,
                    buffer: text.buffer.clone(),
                    color: self.style.font_color,
                });
            }
        }

        self.children
            .iter()
            .for_each(|child| child.collect_instances(instance_data, text_data));
    }
}

impl Node {
    pub fn set_content(mut self, content: &str) -> Self {
        if self.text.is_none() {
            let mut font_system = FontSystem::new();

            self.text = Some(Text {
                buffer: glyphon::Buffer::new(
                    &mut font_system,
                    glyphon::Metrics::new(self.font_size, self.line_height),
                ),
                font_system,
            })
        }

        let family = self.style.font_family.clone();
        let text = self.text.as_mut().unwrap();
        text.buffer.set_text(
            &mut text.font_system,
            content,
            Attrs::new().family(family.as_family()),
            glyphon::Shaping::Advanced,
        );

        self
    }

    pub fn set_font_family(mut self, font_family: FamilyOwned) -> Self {
        self.style.font_family = font_family;
        self
    }

    pub fn set_font_size(mut self, font_size: Units) -> Self {
        self.style.font_size = font_size;
        self
    }

    pub fn set_line_height(mut self, line_height: Units) -> Self {
        self.style.line_height = line_height;
        self
    }

    pub fn set_font_color(mut self, font_color: Color) -> Self {
        self.style.font_color = font_color;
        self
    }

    //pub fn set_font_family(mut self, font_family: Units) -> Self {
    //    self
    //}

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
