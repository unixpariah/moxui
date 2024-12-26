use calc_units::{Context, Units};
use glyphon::{Color, FamilyOwned};
use std::{rc::Rc, sync::RwLock};

use crate::tree::State;

#[repr(C, align(16))]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    rect_pos: [f32; 2],
    rect_size: [f32; 2],

    outline_width: f32,
    outline_offset: f32,

    scale: [f32; 2],
    skew: [f32; 2],
    rotation: f32,

    invert: f32,
    brightness: f32,
    saturate: f32,
    contrast: f32,
    grayscale: f32,
    sepia: f32,
    hue_rotate: f32,

    _padding: [u8; 8],
    rect_color: [f32; 4],
    outline_color: [f32; 4],
    border_radius: [f32; 4],
    border_size: [f32; 4],
    border_top_color: [f32; 4],
    border_right_color: [f32; 4],
    border_bottom_color: [f32; 4],
    border_left_color: [f32; 4],
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

#[derive(PartialEq)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}

pub struct Border {
    pub radius: [f32; 4],
    pub size: [f32; 4],
    pub top_color: [f32; 4],
    pub bottom_color: [f32; 4],
    pub left_color: [f32; 4],
    pub right_color: [f32; 4],
}

impl Default for Border {
    fn default() -> Self {
        Self {
            radius: [0.0, 0.0, 0.0, 0.0],
            top_color: [0.0, 0.0, 0.0, 0.0],
            bottom_color: [0.0, 0.0, 0.0, 0.0],
            left_color: [0.0, 0.0, 0.0, 0.0],
            right_color: [0.0, 0.0, 0.0, 0.0],
            size: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Display {
    Inline,
    Block,
    InlineBlock,
    Contents,
    None,
}

pub struct Style {
    pub top: Units,
    pub right: Units,
    pub bottom: Units,
    pub left: Units,
    pub position: Position,
    pub display: Display,
    pub width: Units,
    pub height: Units,
    pub margin: [Units; 4],
    pub padding: [Units; 4],
    pub border: [Units; 4],
    pub outline_width: Units,
    pub outline_offset: Units,
    pub box_sizing: BoxSizing,
    pub font_color: Color,
    pub font_size: Units,
    pub line_height: Units,
    pub font_family: FamilyOwned,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            top: Units::Auto,
            right: Units::Auto,
            bottom: Units::Auto,
            left: Units::Auto,
            position: Position::Static,
            display: Display::Block,
            width: Units::Auto,
            height: Units::Auto,
            outline_width: Units::Px(0.0),
            outline_offset: Units::Px(0.0),
            margin: [const { Units::Px(0.0) }; 4],
            padding: [const { Units::Px(0.0) }; 4],
            border: [const { Units::Px(0.0) }; 4],
            box_sizing: BoxSizing::ContentBox,
            font_size: Units::Px(16.0),
            font_color: Color::rgb(255, 255, 255),
            line_height: Units::Perc(120.0),
            font_family: FamilyOwned::Serif,
        }
    }
}

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub background_color: [f32; 4],
    pub margin: [f32; 4],
    pub padding: [f32; 4],
    pub border: Border,
    pub outline: Outline,
    pub brightness: f32,
    pub contrast: f32,
    pub grayscale: f32,
    pub hue_rotate: f32,
    pub invert: f32,
    pub saturate: f32,
    pub sepia: f32,
    pub scale: [f32; 2],
    pub rotate: f32,
    pub skew: [f32; 2],
    pub translate: [f32; 2],
    pub font_size: f32,
    pub line_height: f32,

    pub style: Style,

    pub state: Rc<RwLock<State>>,
}

impl Rectangle {
    pub fn new(state: Rc<RwLock<State>>) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            margin: [0.0, 0.0, 0.0, 0.0],
            padding: [0.0, 0.0, 0.0, 0.0],
            background_color: [0.0, 0.0, 0.0, 0.0],
            border: Border::default(),
            outline: Outline::default(),
            brightness: 0.0,
            contrast: 1.0,
            grayscale: 0.0,
            hue_rotate: 0.0,
            invert: 0.0,
            saturate: 1.0,
            sepia: 0.0,
            scale: [1.0, 1.0],
            rotate: 0.0,
            skew: [0.0, 0.0],
            translate: [0.0, 0.0],
            font_size: 16.0,
            line_height: 16.0 * 1.2,

            style: Style::default(),

            state,
        }
    }

    pub fn get_extents(&self) -> Extents {
        if self.style.position == Position::Absolute {
            return Extents {
                x: self.x,
                y: self.y,
                width: 0.0,
                height: 0.0,
            };
        }

        if self.style.position == Position::Fixed {
            let state = self.state.clone();
            let state = state.read().unwrap();
            return Extents {
                x: self.x + state.scroll.0,
                y: self.y + state.scroll.1,
                width: 0.0,
                height: 0.0,
            };
        }

        let (width, height) = match self.style.box_sizing {
            BoxSizing::ContentBox => (
                self.width
                    + self.padding[3]
                    + self.padding[1]
                    + self.border.size[3]
                    + self.border.size[1]
                    + self.margin[3]
                    + self.margin[1],
                self.height
                    + self.padding[0]
                    + self.padding[2]
                    + self.border.size[0]
                    + self.border.size[2]
                    + self.margin[0]
                    + self.margin[2],
            ),
            BoxSizing::BorderBox => (self.width, self.height),
        };

        Extents {
            x: self.x,
            y: self.y,
            width,
            height,
        }
    }

    pub fn get_instance_data(&self) -> InstanceData {
        let extents = self.get_extents();

        let mut x = extents.x + self.margin[3] - self.outline.width - self.outline.offset
            + self.translate[0];
        let mut y = extents.y + self.margin[0] - self.outline.width - self.outline.offset
            + self.translate[1];

        // TODO: children should have access to their state like width height font size
        match self.style.position {
            Position::Sticky => {
                let state = self.state.read().unwrap();
                match (
                    &self.style.top,
                    &self.style.right,
                    &self.style.bottom,
                    &self.style.left,
                ) {
                    (val, Units::Auto, Units::Auto, Units::Auto) => {
                        y = y.max(
                            state.scroll.1
                                + val.to_px(&Context {
                                    root_font_size: state.root_font_size,
                                    parent_size: 0.0,
                                    parent_font_size: 0.0,
                                    viewport: state.viewport,
                                    dpi: state.dpi,
                                    auto: 0.0,
                                }),
                        );
                    }
                    (Units::Auto, Units::Auto, Units::Auto, val) => {
                        x = x.max(
                            state.scroll.0
                                + val.to_px(&Context {
                                    root_font_size: state.root_font_size,
                                    parent_size: 0.0,
                                    parent_font_size: 0.0,
                                    viewport: state.viewport,
                                    dpi: state.dpi,
                                    auto: 0.0,
                                }),
                        );
                    }
                    (Units::Auto, Units::Auto, val, Units::Auto) => {
                        y = (state.viewport.1).max(
                            state.scroll.1
                                + val.to_px(&Context {
                                    root_font_size: state.root_font_size,
                                    parent_size: 0.0,
                                    parent_font_size: 0.0,
                                    viewport: state.viewport,
                                    dpi: state.dpi,
                                    auto: 0.0,
                                }),
                        );
                    }
                    _ => {}
                }
            }
            Position::Relative => {
                let state = self.state.read().unwrap();
                x = self.x
                    + self.style.top.to_px(&Context {
                        root_font_size: state.root_font_size,
                        parent_size: 0.0,
                        parent_font_size: 0.0,
                        viewport: state.viewport,
                        dpi: state.dpi,
                        auto: 0.0,
                    });
                y = self.y
                    + self.style.left.to_px(&Context {
                        root_font_size: state.root_font_size,
                        parent_size: 0.0,
                        parent_font_size: 0.0,
                        viewport: state.viewport,
                        dpi: state.dpi,
                        auto: 0.0,
                    });
            }
            _ => {}
        }

        let width = self.width
            + self.padding[3]
            + self.padding[1]
            + self.border.size[3]
            + self.border.size[1]
            + (self.outline.width + self.outline.offset) * 2.0;

        let height = self.height
            + self.padding[0]
            + self.padding[2]
            + self.border.size[0]
            + self.border.size[2]
            + (self.outline.width + self.outline.offset) * 2.0;

        InstanceData {
            rect_pos: [x, y],
            rect_size: [width, height],

            outline_width: self.outline.width,
            outline_offset: self.outline.offset,

            rotation: self.rotate,
            scale: self.scale,
            skew: self.skew,

            invert: self.invert,
            brightness: self.brightness,
            saturate: self.saturate,
            contrast: self.contrast,
            grayscale: self.grayscale,
            sepia: self.sepia,
            hue_rotate: self.hue_rotate,

            _padding: [const { 0 }; 8],
            rect_color: self.background_color,
            outline_color: self.outline.color,
            border_size: self.border.size,
            border_radius: self.border.radius,
            border_top_color: [
                self.border.top_color[0] * self.border.top_color[3],
                self.border.top_color[1] * self.border.top_color[3],
                self.border.top_color[2] * self.border.top_color[3],
                self.border.top_color[3],
            ],
            border_right_color: [
                self.border.right_color[0] * self.border.right_color[3],
                self.border.right_color[1] * self.border.right_color[3],
                self.border.right_color[2] * self.border.right_color[3],
                self.border.right_color[3],
            ],
            border_bottom_color: [
                self.border.bottom_color[0] * self.border.bottom_color[3],
                self.border.bottom_color[1] * self.border.bottom_color[3],
                self.border.bottom_color[2] * self.border.bottom_color[3],
                self.border.bottom_color[3],
            ],
            border_left_color: [
                self.border.left_color[0] * self.border.left_color[3],
                self.border.left_color[1] * self.border.left_color[3],
                self.border.left_color[2] * self.border.left_color[3],
                self.border.left_color[3],
            ],
        }
    }
}

pub struct Outline {
    pub width: f32,
    pub color: [f32; 4],
    pub offset: f32,
}

impl Default for Outline {
    fn default() -> Self {
        Self {
            color: [0.0, 0.0, 0.0, 0.0],
            width: 0.0,
            offset: 0.0,
        }
    }
}

pub struct Extents {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
