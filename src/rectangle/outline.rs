use super::Rectangle;

// TODO: Implement OutlineStyle

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

impl Rectangle {
    pub fn set_outline_width(mut self, width: f32) -> Self {
        self.outline.width = width;
        self
    }

    pub fn set_outline_offset(mut self, offset: f32) -> Self {
        self.outline.offset = offset;
        self
    }

    pub fn set_outline_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.outline.color = [r, g, b, a];
        self
    }

    pub fn set_outline_style(mut self, style: OutlineStyle) -> Self {
        self.outline.style = style;
        self
    }
}
