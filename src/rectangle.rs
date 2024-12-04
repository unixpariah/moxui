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
pub struct PaddingSize {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl PaddingSize {
    pub fn to_array(&self) -> [f32; 4] {
        [self.top, self.right, self.bottom, self.left]
    }
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

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub background_color: [f32; 4],
    pub padding: PaddingSize,
    pub box_sizing: BoxSizing,
    pub border: Border,
    pub outline: Outline,
    pub box_shadow: BoxShadow,
    pub blur: f32,
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
                    + self.border.size.right,
                self.height
                    + self.padding.top
                    + self.padding.bottom
                    + self.border.size.top
                    + self.border.size.bottom,
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

    pub fn get_instance(&self) -> buffers::Instance {
        let extents = self.get_extents();

        let x = extents.x - self.outline.width - self.outline.offset;

        let y = extents.y - self.outline.width - self.outline.offset;

        let width = extents.width + (self.outline.width + self.outline.offset) * 2.0;

        let height = extents.height + (self.outline.width + self.outline.offset) * 2.0;

        // TODO: calculate size of shadow and get max of either outline width + outline.offset or
        // the calculated shadow (cant just take offset as blurring kind makes it different size)

        let c = self.background_color;

        buffers::Instance {
            dimensions: [x, y, width, height],
            color: [c[0] * c[3], c[1] * c[3], c[2] * c[3], c[3]], // Premultiply colors
            border_radius: self.border.radius.to_array(),
            border_size: self.border.size.to_array(),
            border_color: self.border.color,
            outline: [self.outline.width, self.outline.offset],
            outline_color: self.outline.color,
            filter: [self.brightness, self.saturate, self.contrast, self.invert],
            grayscale: self.grayscale,
            scale: self.scale,
            rotation: self.rotate,
            translate: self.translate,
            skew: self.skew,
        }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            blur: 0.0,
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            padding: PaddingSize::default(),
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
