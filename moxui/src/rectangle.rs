mod flexbox;

use std::{rc::Rc, sync::RwLock};

use crate::buffers;
use calc_units::Units;
use flexbox::{
    AlignContent, AlignItems, AlignSelf, FlexBasis, FlexDirection, FlexGrow, FlexShrink, FlexWrap,
    JustifyContent, Order,
};

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
    border_color_top: [f32; 4],
    border_color_right: [f32; 4],
    border_color_bottom: [f32; 4],
    border_color_left: [f32; 4],
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

pub enum Float {
    Left,
    Right,
    None,
}

#[derive(PartialEq)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}

pub enum BorderStyle {
    None,
    Solid,
    Dotted,
    Dashed,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
    Hidden,
}

pub struct Border {
    pub radius: [f32; 4],
    pub size: [f32; 4],
    pub color: [f32; 4],
    pub style: BorderStyle,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            radius: [0.0, 0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0, 0.0],
            size: [0.0, 0.0, 0.0, 0.0],
            style: BorderStyle::Solid,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Display {
    Inline,
    Block,
    InlineBlock,
    Flex,
    Contents,
    InlineFlex,
    Grid,
    InlineGrid,
    Table,
    InlineTable,
    ListItem,
    RunIn,
    None,
}

pub struct Style {
    pub position: Position,
    pub float: Float,
    pub display: Display,
    pub x: Units,
    pub y: Units,
    pub width: Option<Units>,
    pub height: Option<Units>,
    pub margin: [Units; 4],
    pub padding: [Units; 4],
    pub border: [Units; 4],
    pub box_sizing: BoxSizing,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    pub align_self: AlignSelf,
    pub order: Order,
    pub flex_grow: FlexGrow,
    pub flex_shrink: FlexShrink,
    pub flex_basis: FlexBasis,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            position: Position::Static,
            float: Float::None,
            display: Display::Block,
            x: Units::Px(0.0),
            y: Units::Px(0.0),
            width: None,
            height: None,
            margin: [const { Units::Px(0.0) }; 4],
            padding: [const { Units::Px(0.0) }; 4],
            border: [const { Units::Px(0.0) }; 4],
            box_sizing: BoxSizing::ContentBox,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Nowrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            align_content: AlignContent::Stretch,
            align_self: AlignSelf::Auto,
            order: Order(0),
            flex_grow: FlexGrow(0),
            flex_shrink: FlexShrink(1),
            flex_basis: FlexBasis::Auto,
        }
    }
}

pub struct State {
    pub viewport: (f32, f32),
    pub scroll: (f32, f32),
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

        let x = extents.x + self.margin[3] - self.outline.width - self.outline.offset
            + self.translate[0];
        let y = extents.y + self.margin[0] - self.outline.width - self.outline.offset
            + self.translate[1];

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

        let bg = self.background_color;
        let oc = self.outline.color;
        let bc = self.border.color;

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

            _padding: [0, 0, 0, 0, 0, 0, 0, 0],
            rect_color: bg,
            outline_color: oc,
            border_size: self.border.size,
            border_radius: self.border.radius,
            border_color_top: bc,
            border_color_right: bc,
            border_color_bottom: bc,
            border_color_left: bc,
        }
    }

    pub fn get_instance(&self) -> buffers::Instance {
        buffers::Instance { color: 0.0 }
    }
}

pub enum OutlineStyle {
    None,
    Solid,
    Dotted,
    Dashed,
    Double,
    Groove,
    Ridge,
    Hidden,
}

pub struct Outline {
    pub width: f32,
    pub color: [f32; 4],
    pub style: OutlineStyle,
    pub offset: f32,
}

impl Default for Outline {
    fn default() -> Self {
        Self {
            color: [0.0, 0.0, 0.0, 0.0],
            width: 0.0,
            style: OutlineStyle::Solid,
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
