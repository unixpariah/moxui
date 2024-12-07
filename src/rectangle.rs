use crate::buffers;

#[derive(PartialEq)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}

#[derive(Default)]
pub struct BoxShadow {
    pub x_offset: f32,
    pub y_offset: f32,
    pub softness: f32,
    pub color: [f32; 4],
    pub inset: bool,
}

#[derive(Default)]
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
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

#[derive(Default)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

impl BorderRadius {
    pub fn to_array(&self) -> [f32; 4] {
        [
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right,
        ]
    }
}

#[derive(Default)]
pub struct BorderSize {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl BorderSize {
    pub fn to_array(&self) -> [f32; 4] {
        [self.top, self.right, self.bottom, self.left]
    }
}

pub struct Border {
    pub radius: BorderRadius,
    pub size: BorderSize,
    pub color: [f32; 4],
    pub style: BorderStyle,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            radius: BorderRadius::default(),
            color: [0.0, 0.0, 0.0, 0.0],
            size: BorderSize::default(),
            style: BorderStyle::Solid,
        }
    }
}

#[derive(PartialEq)]
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
    None,
    RunIn,
}

pub struct Rectangle {
    pub display: Display,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub background_color: [f32; 4],
    pub margin: Spacing,
    pub padding: Spacing,
    pub box_sizing: BoxSizing,
    pub border: Border,
    pub outline: Outline,
    pub box_shadow: BoxShadow,
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

impl Rectangle {
    pub fn get_extents(&self) -> Extents {
        let (width, height) = match self.box_sizing {
            BoxSizing::ContentBox => (
                self.width
                    + self.padding.left
                    + self.padding.right
                    + self.border.size.left
                    + self.border.size.right
                    + self.margin.right,
                self.height
                    + self.padding.top
                    + self.padding.bottom
                    + self.border.size.top
                    + self.border.size.bottom
                    + self.margin.bottom,
            ),
            BoxSizing::BorderBox => (self.width, self.height),
        };

        Extents {
            x: self.x + self.margin.left,
            y: self.y + self.margin.top,
            width,
            height,
        }
    }

    pub fn get_instance(&self) -> buffers::Instance {
        let extents = self.get_extents();

        let x = extents.x - self.outline.width - self.outline.offset;

        let y = extents.y - self.outline.width - self.outline.offset;

        let width =
            extents.width - self.margin.right + (self.outline.width + self.outline.offset) * 2.0;

        let height =
            extents.height - self.margin.bottom + (self.outline.width + self.outline.offset) * 2.0;

        let bg = self.background_color;
        let oc = self.outline.color;
        let bc = self.border.color;

        buffers::Instance {
            dimensions: [x, y, width, height],
            color: [bg[0] * bg[3], bg[1] * bg[3], bg[2] * bg[3], bg[3]],
            border_radius: self.border.radius.to_array(),
            border_size: self.border.size.to_array(),
            border_color: [bc[0] * bc[3], bc[1] * bc[3], bc[2] * bc[3], bc[3]],
            outline: [self.outline.width, self.outline.offset],
            outline_color: [oc[0] * oc[3], oc[1] * oc[3], oc[2] * oc[3], oc[3]],
            filter: [self.brightness, self.saturate, self.contrast, self.invert],
            grayscale: self.grayscale,
            scale: self.scale,
            rotation: self.rotate,
            translate: self.translate,
            skew: self.skew,
            sepia: self.sepia,
            hue_rotate: self.hue_rotate,
        }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            display: Display::Block,
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            margin: Spacing::default(),
            padding: Spacing::default(),
            background_color: [0.0, 0.0, 0.0, 0.0],
            border: Border::default(),
            outline: Outline::default(),
            box_sizing: BoxSizing::ContentBox,
            box_shadow: BoxShadow::default(),
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
        }
    }
}
